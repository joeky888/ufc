use std::{
    env,
    io::{BufRead, BufReader, Write},
    process::{self, Child, Command, Stdio},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, SystemTime},
};

use fancy_regex::Regex;
use lazy_static::lazy_static;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

lazy_static! {
    // Global SETTINGS
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings {
        subcommand_name: String::new(),
        subcommand_start: SystemTime::now(),
        watch_sec: 0,
        time: false,
        palettes: vec![],
    });
}

#[derive(Debug)]
pub struct Settings {
    pub subcommand_name: String,
    pub subcommand_start: SystemTime,
    pub watch_sec: u64,
    pub time: bool,
    pub palettes: Vec<Palette<'static>>,
}

#[derive(Debug)]
struct ColorString<'a> {
    text: String,
    color: &'a Colours,
}

#[derive(Debug)]
pub struct Palette<'a> {
    pub regexp: Regex,
    pub colours: Vec<&'a Colours>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Colours {
    UnChanged,
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
    BlackOnBlack,
    BlackOnBlue,
    BlackOnGreen,
    BlackOnRed,
    BlackOnCyan,
    BlackOnMagenta,
    BlackOnYellow,
    BlackOnWhite,
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

fn clear_screen() {
    if cfg!(windows) {
        let _ = Command::new("cmd.exe").args(&["/c", "cls"]).status();
    } else {
        // https://stackoverflow.com/a/66911945
        print!("{esc}c", esc = 27 as char);
    };
}

fn process_exit(exit_code: i32) {
    if !SETTINGS.read().unwrap().time {
        process::exit(exit_code);
    }
    match SETTINGS.read().unwrap().subcommand_start.elapsed() {
        Ok(elapsed) => {
            println!("\nThe subcommand takes {:?} to finish", elapsed);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    process::exit(exit_code);
}

pub fn pre_exec(palettes: Vec<Palette<'static>>) {
    SETTINGS.write().unwrap().palettes = palettes;
    let ctrlc_hit = Arc::new(RwLock::new(false));
    let setting = SETTINGS.read().unwrap();
    let arg_start = env::args()
        .position(|cmd| cmd.eq(&setting.subcommand_name))
        .unwrap();

    let mut subcommand_proc = Arc::new(RwLock::new(Command::new("true").spawn().expect("")));
    let child_clone = Arc::clone(&subcommand_proc);
    let ctrlc_hit_clone = Arc::clone(&ctrlc_hit);

    ctrlc::set_handler(move || {
        // println!("ctrlc hit!");
        match child_clone.write().unwrap().kill() {
            // Ignore kill() error, because the program exits anyway
            Err(_) => {}
            Ok(_) => {}
        }
        // println!("ctrlc hit end!");
        *ctrlc_hit_clone.write().unwrap() = true;
        // If the program does not stop after 100ms, e.g blocked by thread::sleep, then force quit
        // This is required because some program will output its last words before exiting
        thread::sleep(Duration::from_millis(100));
        process_exit(0);
    })
    .unwrap();

    let mut exit_code = 0;
    if setting.watch_sec != 0 {
        while !*ctrlc_hit.read().unwrap() {
            clear_screen();
            exit_code = exec(arg_start, &mut subcommand_proc);
            thread::sleep(Duration::from_secs(setting.watch_sec));
        }
    } else {
        exit_code = exec(arg_start, &mut subcommand_proc);
    }
    process_exit(exit_code);
}

fn exec(arg_start: usize, subcommand_proc: &mut Arc<RwLock<Child>>) -> i32 {
    // let palettes = SETTINGS.read().unwrap().palettes;
    let args: Vec<String> = env::args().collect();

    *subcommand_proc = Arc::new(RwLock::new(
        Command::new(args[arg_start].as_str())
            .args(&args[arg_start + 1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    ));

    let stdout = BufReader::new(subcommand_proc.write().unwrap().stdout.take().unwrap());
    let stderr = BufReader::new(subcommand_proc.write().unwrap().stderr.take().unwrap());

    // let palettes_stdout = Arc::new(palettes);
    // let palettes_stderr = palettes_stdout.clone();

    // Start to capture and color stdout
    let stdout_thread = thread::spawn(move || {
        stdout.lines().for_each(|line| {
            let bufwtr = BufferWriter::stdout(ColorChoice::Always);
            let ln = &line.unwrap();
            color_std(&bufwtr, ln);
        });
    });

    // Start to capture and color stderr
    let stderr_thread = thread::spawn(move || {
        stderr.lines().for_each(|line| {
            let bufwtr = BufferWriter::stderr(ColorChoice::Always);
            let ln = &line.unwrap();
            color_std(&bufwtr, &ln);
        });
    });

    // Clone the process to the ctrlc thread (to be killed)

    // let child_clone = child.clone();
    // SETTINGS.write().unwrap().subcommand_proc = None;
    // SETTINGS.write().unwrap().watch_sec = 2;
    // SETTINGS.write().unwrap().subcommand_proc = Arc::clone(&child);

    // ctrlc::set_handler(move || {
    //     // Ignore kill() error, because the program exits anyway
    //     match child_clone.write().unwrap().kill() {
    //         Err(_) => (),
    //         Ok(_) => (),
    //     }
    //     SETTINGS.write().unwrap().ctrlc_hit = true;
    // })
    // .unwrap();

    let status = subcommand_proc.write().unwrap().wait().unwrap();
    let exit_code = match status.code() {
        Some(code) => code,
        None => 0,
    };

    // For some reason, we have to wait a longer here to make sure the sub program exits
    // And to correctly capture the last word of the sub program
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    return exit_code;
}

fn color_std(bufwtr: &BufferWriter, ln: &String) {
    let mut buffer = bufwtr.buffer();
    let mut buffer_writer = bufwtr.buffer();

    let mut main_string = vec![ColorString {
        text: ln.clone(),
        color: &Colours::Default,
    }];
    let main_string = colored_output(&mut main_string);

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
}

fn get_color(color: &Colours) -> ColorSpec {
    let mut col = ColorSpec::new();
    match color {
        Colours::UnChanged => col // This one should be unreable
            .set_fg(Some(Color::Magenta))
            .set_intense(true)
            .set_underline(true),
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
        Colours::BlackOnBlack => col.set_bg(Some(Color::Black)).set_fg(Some(Color::Black)),
        Colours::BlackOnBlue => col.set_bg(Some(Color::Blue)).set_fg(Some(Color::Black)),
        Colours::BlackOnGreen => col.set_bg(Some(Color::Green)).set_fg(Some(Color::Black)),
        Colours::BlackOnRed => col.set_bg(Some(Color::Red)).set_fg(Some(Color::Black)),
        Colours::BlackOnCyan => col.set_bg(Some(Color::Cyan)).set_fg(Some(Color::Black)),
        Colours::BlackOnMagenta => col.set_bg(Some(Color::Magenta)).set_fg(Some(Color::Black)),
        Colours::BlackOnYellow => col.set_bg(Some(Color::Yellow)).set_fg(Some(Color::Black)),
        Colours::BlackOnWhite => col.set_bg(Some(Color::White)).set_fg(Some(Color::Black)),
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

fn colored_output<'a>(main_string: &'a mut Vec<ColorString<'a>>) -> &'a Vec<ColorString<'a>> {
    let mut prev_color = &Colours::Default;
    for palette in SETTINGS.read().unwrap().palettes.iter() {
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
                    for (i, capture) in captures.iter().enumerate() {
                        if i == 0 {
                            let mut color = palette.colours[0];
                            if color == &Colours::UnChanged {
                                color = prev_color;
                            }
                            colored_strings.push(ColorString {
                                text: String::from_str(&str[last_start..full_match_end]).unwrap(),
                                color: color,
                            });
                            prev_color = color;
                            continue;
                        }

                        match capture {
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

                        let start = capture.unwrap().start();
                        let end = capture.unwrap().end();

                        let mut color = palette.colours[0];
                        if color == &Colours::UnChanged {
                            color = prev_color;
                        }

                        colored_strings.push(ColorString {
                            text: String::from_str(&str[last_start..start]).unwrap(),
                            color: &color,
                        });
                        prev_color = color;

                        if i < palette.colours.len() {
                            color = palette.colours[i];
                            if color == &Colours::UnChanged {
                                color = prev_color;
                            }
                        }
                        colored_strings.push(ColorString {
                            text: String::from_str(&str[start..end]).unwrap(),
                            color: color,
                        });
                        prev_color = color;

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
    // main_string.retain(|color_string| color_string.text != "");
    // println!("main_string={:?}", main_string);
    main_string
}
