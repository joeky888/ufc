use std::{
    env,
    io::{BufRead, BufReader, Write},
    process::{self, Command, Stdio},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

use fancy_regex::Regex;
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

#[allow(dead_code)]
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
    // BOld
    BDefault,
    BBlack,
    BBlue,
    BGreen,
    BRed,
    BCyan,
    BMagenta,
    BYellow,
    BWhite,
    // Dark
    DDefault,
    DBlack,
    DBlue,
    DGreen,
    DRed,
    DCyan,
    DMagenta,
    DYellow,
    DWhite,
    // Backgroud Color
    OnBlack,
    OnBlue,
    OnGreen,
    OnRed,
    OnCyan,
    OnMagenta,
    OnYellow,
    OnWhite,
    // Undoerline
    UDefault,
    UBlack,
    UBlue,
    UGreen,
    URed,
    UCyan,
    UMagenta,
    UYellow,
    UWhite,
    // Underline Bold
    UBDefault,
    UBBlack,
    UBBlue,
    UBGreen,
    UBRed,
    UBCyan,
    UBMagenta,
    UBYellow,
    UBWhite,
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
        Colours::BDefault => col.set_bold(true).set_fg(None),
        Colours::Black => col.set_fg(Some(Color::Black)),
        Colours::Blue => col.set_fg(Some(Color::Blue)),
        Colours::Green => col.set_fg(Some(Color::Green)),
        Colours::Red => col.set_fg(Some(Color::Red)),
        Colours::Cyan => col.set_fg(Some(Color::Cyan)),
        Colours::Magenta => col.set_fg(Some(Color::Magenta)),
        Colours::Yellow => col.set_fg(Some(Color::Yellow)),
        Colours::White => col.set_fg(Some(Color::White)),
        Colours::BBlack => col.set_bold(true).set_fg(Some(Color::Ansi256(8))),
        Colours::BBlue => col.set_bold(true).set_fg(Some(Color::Blue)),
        Colours::BGreen => col.set_bold(true).set_fg(Some(Color::Green)),
        Colours::BRed => col.set_bold(true).set_fg(Some(Color::Red)),
        Colours::BCyan => col.set_bold(true).set_fg(Some(Color::Cyan)),
        Colours::BMagenta => col.set_bold(true).set_fg(Some(Color::Magenta)),
        Colours::BYellow => col.set_bold(true).set_fg(Some(Color::Yellow)),
        Colours::BWhite => col.set_bold(true).set_fg(Some(Color::White)),
        Colours::UDefault => col.set_underline(true).set_fg(None),
        Colours::UBlack => col.set_underline(true).set_fg(Some(Color::Black)),
        Colours::UBlue => col.set_underline(true).set_fg(Some(Color::Blue)),
        Colours::UGreen => col.set_underline(true).set_fg(Some(Color::Green)),
        Colours::URed => col.set_underline(true).set_fg(Some(Color::Red)),
        Colours::UCyan => col.set_underline(true).set_fg(Some(Color::Cyan)),
        Colours::UMagenta => col.set_underline(true).set_fg(Some(Color::Magenta)),
        Colours::UYellow => col.set_underline(true).set_fg(Some(Color::Yellow)),
        Colours::UWhite => col.set_underline(true).set_fg(Some(Color::White)),
        Colours::UBBlack => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Black)),
        Colours::UBBlue => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Blue)),
        Colours::UBGreen => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Green)),
        Colours::UBRed => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Red)),
        Colours::UBCyan => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Cyan)),
        Colours::UBMagenta => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Magenta)),
        Colours::UBYellow => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Yellow)),
        Colours::UBWhite => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::White)),
        Colours::UBDefault => col.set_underline(true).set_bold(true).set_fg(None),
        Colours::OnBlack => col.set_bg(Some(Color::Black)),
        Colours::OnBlue => col.set_bg(Some(Color::Blue)),
        Colours::OnGreen => col.set_bg(Some(Color::Green)),
        Colours::OnRed => col.set_bg(Some(Color::Red)),
        Colours::OnCyan => col.set_bg(Some(Color::Cyan)),
        Colours::OnMagenta => col.set_bg(Some(Color::Magenta)),
        Colours::OnYellow => col.set_bg(Some(Color::Yellow)),
        Colours::OnWhite => col.set_bg(Some(Color::White)),
        Colours::DDefault => col.set_dimmed(true).set_fg(None),
        Colours::DBlack => col.set_dimmed(true).set_fg(Some(Color::Black)),
        Colours::DBlue => col.set_dimmed(true).set_fg(Some(Color::Blue)),
        Colours::DGreen => col.set_dimmed(true).set_fg(Some(Color::Green)),
        Colours::DRed => col.set_dimmed(true).set_fg(Some(Color::Red)),
        Colours::DCyan => col.set_dimmed(true).set_fg(Some(Color::Cyan)),
        Colours::DMagenta => col.set_dimmed(true).set_fg(Some(Color::Magenta)),
        Colours::DYellow => col.set_dimmed(true).set_fg(Some(Color::Yellow)),
        Colours::DWhite => col.set_dimmed(true).set_fg(Some(Color::White)),
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
                .unwrap()
            {
                Some(captures) => {
                    let str = main_string[index].text.as_str();
                    let mut colored_strings: Vec<ColorString> = vec![];

                    // Non-matched start
                    let start = 0;
                    let end = captures.get(0).unwrap().start();
                    colored_strings.push(ColorString {
                        text: String::from_str(&str[start..end]).unwrap(),
                        color: &Colours::Default,
                    });

                    // captures[0] -> Full match
                    // captures[1..] -> Group match
                    let mut last_start = captures.get(0).unwrap().start();
                    let full_match_end = captures.get(0).unwrap().end();
                    let mut is_full_match = false;
                    for (i, _capture) in captures.iter().enumerate() {
                        if i == 0 {
                            colored_strings.push(ColorString {
                                text: String::from_str(&str[last_start..full_match_end]).unwrap(),
                                color: palette.colours[0],
                            });
                            continue;
                        }

                        // println!("captures={:?}", captures);
                        match captures.get(i) {
                            Some(_) => {
                                if !is_full_match {
                                    colored_strings.pop();
                                }
                                is_full_match = true;
                            }
                            None => {
                                continue;
                            }
                        }

                        let start = captures.get(i).unwrap().start();
                        let end = captures.get(i).unwrap().end();

                        colored_strings.push(ColorString {
                            text: String::from_str(&str[last_start..start]).unwrap(),
                            color: palette.colours[0],
                        });

                        colored_strings.push(ColorString {
                            text: String::from_str(&str[start..end]).unwrap(),
                            color: palette.colours[i],
                        });

                        last_start = end;
                    }

                    // Non-matched end
                    if !is_full_match {
                        last_start = captures.get(0).unwrap().end();
                    }

                    colored_strings.push(ColorString {
                        text: String::from_str(&str[last_start..]).unwrap(),
                        color: &Colours::Default,
                    });

                    // println!("colored_strings={:?}", colored_strings);
                    let len = colored_strings.len();

                    main_string[index].text = String::new();
                    main_string.remove(index);
                    main_string.splice((index)..(index), colored_strings);
                    index += len;
                }
                None => {}
            };
            index += 1;
        }
    }

    // Remove empty strings
    // main_string.retain(|color_string| color_string.text != "");
    // println!("{:?}", main_string);
    main_string
}
