use std::{
    env,
    io::{BufRead, BufReader, Write},
    process::{self, Command, Stdio},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

use regex::Regex;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

#[derive(Debug)]
struct ColorString<'a> {
    text: String,
    color: &'a Colours,
}

pub struct Palette<'a> {
    pub regexp: Regex,
    pub colours: Vec<&'a Colours>,
}

#[derive(Debug, PartialEq)]
pub enum Colours {
    Default,
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    BoldDefault,
    BoldBlack,
    BoldBlue,
    BoldGreen,
    BoldRed,
    BoldCyan,
    BoldMagenta,
    BoldYellow,
    BoldWhite,
    UnderlineDefault,
    UnderlineBlack,
    UnderlineBlue,
    UnderlineGreen,
    UnderlineRed,
    UnderlineCyan,
    UnderlineMagenta,
    UnderlineYellow,
    UnderlineWhite,
    UnderlineBoldDefault,
    UnderlineBoldBlack,
    UnderlineBoldBlue,
    UnderlineBoldGreen,
    UnderlineBoldRed,
    UnderlineBoldCyan,
    UnderlineBoldMagenta,
    UnderlineBoldYellow,
    UnderlineBoldWhite,
}

pub fn exec(palettes: Vec<Palette<'static>>) {
    let args: Vec<String> = env::args().collect();

    let child = Arc::new(RwLock::new(
        Command::new(args[1].as_str())
            .args(&args[2..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    ));

    let stdout = BufReader::new(child.write().unwrap().stdout.take().unwrap());
    let stderr = BufReader::new(child.write().unwrap().stderr.take().unwrap());

    // Start to capture stdout
    let stdout_thread = thread::spawn(move || {
        stdout.lines().for_each(|line| {
            let bufwtr = BufferWriter::stdout(ColorChoice::Always);
            let mut buffer_writer = bufwtr.buffer();
            let ln = &line.unwrap();
            let mut buffer = bufwtr.buffer();

            let mut main_string = vec![ColorString {
                text: ln.clone(),
                color: &Colours::Default,
            }];
            let main_string = colored_output(&mut main_string, &palettes);

            for str in main_string.iter() {
                buffer.set_color(&get_color(str.color)).unwrap();
                write!(&mut buffer, "{}", str.text).unwrap();

                // Reset color
                buffer.set_color(&get_color(&Colours::Default)).unwrap();
            }

            write!(&mut buffer, "\n").unwrap();
            buffer_writer.write(&buffer.as_slice().to_vec()).unwrap();
            // buffer_writer.write(&buffer2.as_slice().to_vec()).unwrap();
            bufwtr.print(&buffer_writer).unwrap();
        });
    });

    // Start to capture stderr
    let stderr_thread = thread::spawn(move || {
        stderr.lines().for_each(|line| {
            println!("{}", line.unwrap());
        });
    });

    // Clone the process to the ctrlc thread (to be killed)
    let child_clone = Arc::clone(&child);
    ctrlc::set_handler(move || {
        // Ignore kill() error, because the program exits anyway
        match child_clone.write().unwrap().kill() {
            Err(_) => (),
            Ok(_) => (),
        }
    })
    .unwrap();

    let status = child.write().unwrap().wait().unwrap();
    let exit_code = match status.code() {
        Some(code) => code,
        None => 0,
    };

    // For some reason, we have to wait a longer here to make sure the sub program exits
    // And to correctly capture the last word of the sub program
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    process::exit(exit_code);
}

fn get_color(color: &Colours) -> ColorSpec {
    let mut col = ColorSpec::new();
    match color {
        Colours::Default => col.set_fg(None),
        Colours::BoldDefault => col.set_bold(true).set_fg(None),
        Colours::Black => col.set_fg(Some(Color::Black)),
        Colours::Blue => col.set_fg(Some(Color::Blue)),
        Colours::Green => col.set_fg(Some(Color::Green)),
        Colours::Red => col.set_fg(Some(Color::Red)),
        Colours::Cyan => col.set_fg(Some(Color::Cyan)),
        Colours::Magenta => col.set_fg(Some(Color::Magenta)),
        Colours::Yellow => col.set_fg(Some(Color::Yellow)),
        Colours::White => col.set_fg(Some(Color::White)),
        Colours::BoldBlack => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldBlue => col.set_bold(true).set_fg(Some(Color::Blue)),
        Colours::BoldGreen => col.set_bold(true).set_fg(Some(Color::Green)),
        Colours::BoldRed => col.set_bold(true).set_fg(Some(Color::Red)),
        Colours::BoldCyan => col.set_bold(true).set_fg(Some(Color::Cyan)),
        Colours::BoldMagenta => col.set_bold(true).set_fg(Some(Color::Magenta)),
        Colours::BoldYellow => col.set_bold(true).set_fg(Some(Color::Yellow)),
        Colours::BoldWhite => col.set_bold(true).set_fg(Some(Color::White)),
        Colours::UnderlineDefault => col.set_underline(true).set_fg(None),
        Colours::UnderlineBoldDefault => col.set_underline(true).set_bold(true).set_fg(None),
        Colours::UnderlineBlack => col.set_underline(true).set_fg(Some(Color::Black)),
        Colours::UnderlineBlue => col.set_underline(true).set_fg(Some(Color::Blue)),
        Colours::UnderlineGreen => col.set_underline(true).set_fg(Some(Color::Green)),
        Colours::UnderlineRed => col.set_underline(true).set_fg(Some(Color::Red)),
        Colours::UnderlineCyan => col.set_underline(true).set_fg(Some(Color::Cyan)),
        Colours::UnderlineMagenta => col.set_underline(true).set_fg(Some(Color::Magenta)),
        Colours::UnderlineYellow => col.set_underline(true).set_fg(Some(Color::Yellow)),
        Colours::UnderlineWhite => col.set_underline(true).set_fg(Some(Color::White)),
        Colours::UnderlineBoldBlack => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Black)),
        Colours::UnderlineBoldBlue => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Blue)),
        Colours::UnderlineBoldGreen => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Green)),
        Colours::UnderlineBoldRed => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Red)),
        Colours::UnderlineBoldCyan => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Cyan)),
        Colours::UnderlineBoldMagenta => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Magenta)),
        Colours::UnderlineBoldYellow => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Yellow)),
        Colours::UnderlineBoldWhite => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::White)),
    };
    col
}

fn colored_output<'a>(
    main_string: &'a mut Vec<ColorString<'a>>,
    palettes: &'a Vec<Palette>,
) -> &'a Vec<ColorString<'a>> {
    for palette in palettes.iter() {
        let mut index = 0;
        // Instead of using a for loop, the size of main_string will grow so we have to use while loop
        // https://stackoverflow.com/questions/47338839
        while index < main_string.len() {
            // println!("i={} main_string.len()={}", index, main_string.len());

            if !main_string[index].color.eq(&Colours::Default) {
                index += 1;
                continue; // Ignore those already been colored
            }

            match palette
                .regexp
                .captures(main_string[index].text.clone().as_str())
            {
                Some(captures) => {
                    // println!("!!!!i={} {:?}!!!", index, captures);

                    let str = main_string[index].text.as_str();
                    // println!("str={:?}!!!", str);
                    let mut colored_strings: Vec<ColorString> = vec![];

                    // Non-matched start
                    let start = 0;
                    let end = captures.get(0).unwrap().start();
                    colored_strings.push(ColorString {
                        text: String::from_str(&str[start..end]).unwrap(),
                        color: &Colours::Default,
                    });

                    // captures[0] -> Full match
                    let start = captures.get(0).unwrap().start();
                    let end = captures.get(0).unwrap().end();
                    colored_strings.push(ColorString {
                        text: String::from_str(&str[start..end]).unwrap(),
                        color: palette.colours[0],
                    });

                    // captures[1..] -> Group match
                    let mut new_start = captures.get(0).unwrap().start();
                    let mut new_end = captures.get(0).unwrap().end();
                    for (i, _capture) in captures.iter().enumerate() {
                        if i == 0 {
                            continue; // Ignore because it is a full match and is already done.
                        }

                        if i == 1 {
                            colored_strings.pop();
                        }
                        // println!("captures={:?}", captures);
                        match captures.get(i) {
                            Some(_) => (),
                            None => {
                                continue;
                            }
                        }
                        let before_start = new_start;
                        let before_end = captures.get(i).unwrap().start();
                        let start = captures.get(i).unwrap().start();
                        let end = captures.get(i).unwrap().end();
                        let after_start = captures.get(i).unwrap().end();
                        let after_end = new_end;
                        // println!("str={}", str);
                        // println!("before_start={},before_end={},start={},end={},after_start={},after_end={}",before_start,before_end,start,end,after_start,after_end);

                        colored_strings.push(ColorString {
                            text: String::from_str(&str[before_start..before_end]).unwrap(),
                            color: palette.colours[0],
                        });
                        colored_strings.push(ColorString {
                            text: String::from_str(&str[start..end]).unwrap(),
                            color: palette.colours[i],
                        });

                        if i == captures.len() - 1 {
                            // Push the last one (The rest of the string) back when the for loop ends
                            // Because the for loop ends here, so we don't need to split the rest of the string anymore
                            colored_strings.push(ColorString {
                                text: String::from_str(&str[after_start..after_end]).unwrap(),
                                color: palette.colours[0],
                            });
                        }

                        // println!("colored_strings={:?}", colored_strings);
                        new_start = after_start;
                        new_end = after_end;
                    }

                    // Non-matched end
                    let start = captures.get(0).unwrap().end();
                    colored_strings.push(ColorString {
                        text: String::from_str(&str[start..]).unwrap(),
                        color: &Colours::Default,
                    });

                    // println!("colored_strings={:?}", colored_strings);

                    main_string[index].text = String::new();
                    main_string.remove(index);
                    main_string.splice((index)..(index), colored_strings);
                    index += 1;
                }
                None => {}
            };
            index += 1;
        }
    }

    // Remove empty strings
    main_string.retain(|color_string| color_string.text != "");
    // println!("{:?}", main_string);
    main_string
}
