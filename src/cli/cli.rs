use std::{
    env,
    io::{BufRead, BufReader, Write},
    num::ParseFloatError,
    process::{
        Child, Command, Stdio, {self},
    },
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, SystemTime},
};

use atty::Stream;
use clap::{AppSettings, Clap};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use termcolor::{BufferWriter, BufferedStandardStream, Color, ColorChoice, ColorSpec, WriteColor};

lazy_static! {
    // Global SETTINGS
    pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings {
        clap_args: Opts{
            watch: 0.0,
            time: false,
            boost: false,
            nocolor: false,
            universal: false,
        },
        subcommand_name: String::new(),
        subcommand_start: SystemTime::now(),
        palettes: vec![],
        is_tty: atty::is(Stream::Stdout),
    });
}

#[derive(Debug)]
pub struct Settings {
    pub clap_args: Opts,
    pub subcommand_name: String,
    pub subcommand_start: SystemTime,
    pub palettes: Vec<Palette<'static>>,
    pub is_tty: bool,
}

#[derive(Debug, Clap)]
#[clap(
    name = "ufc",
    setting = AppSettings::AllowExternalSubcommands,
    setting = AppSettings::SubcommandRequiredElseHelp,
    global_setting = AppSettings::ColoredHelp,
    global_setting = AppSettings::DisableVersionForSubcommands,
    global_setting = AppSettings::DisableHelpSubcommand,
    global_setting = AppSettings::DisableHelpFlag,
)]
pub struct Opts {
    #[clap(
        short = 'w',
        long = "watch",
        parse(try_from_str = parse_watch_duration),
        default_value = "0",
    )]
    pub watch: f64,

    #[clap(short = 't', long = "time")]
    pub time: bool,

    #[clap(short = 'b', long = "boost")]
    pub boost: bool,

    #[clap(short = 'n', long = "nocolor")]
    pub nocolor: bool,

    #[clap(short = 'u', long = "universal")]
    pub universal: bool,
}

fn parse_watch_duration(src: &str) -> Result<f64, ParseFloatError> {
    let time_re =
        Regex::new(r#"((\d*\.?\d*)[h|H])?((\d*\.?\d*)[m|M])?((\d*\.?\d*)[s|S])?"#).unwrap();
    let captures = time_re.captures(src).unwrap().unwrap();
    let h = captures
        .get(2)
        .map_or(0.0, |v| v.as_str().to_string().parse().unwrap_or(0.0));
    let m = captures
        .get(4)
        .map_or(0.0, |v| v.as_str().to_string().parse().unwrap_or(0.0));
    let s = captures
        .get(6)
        .map_or(0.0, |v| v.as_str().to_string().parse().unwrap_or(0.0));
    // println!("h:{} m:{} s:{}", h, m, s);
    let duration = h * 3600.0 + m * 60.0 + s;
    if duration != 0.0 {
        Ok(duration) // hhmmss format
    } else {
        src.parse() // ss format
    }
}

#[derive(Debug)]
struct ColorString<'a> {
    text: String,
    color: &'a Colors,
}

#[derive(Debug)]
pub struct Palette<'a> {
    pub regexp: Regex,
    pub colors: Vec<&'a Colors>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Colors {
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
    if !SETTINGS.read().unwrap().is_tty && cfg!(windows) {
        let _ = Command::new("cmd.exe").args(&["/c", "cls"]).status();
    } else {
        // https://stackoverflow.com/a/34837038
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

fn process_exit(exit_code: i32) {
    if !SETTINGS.read().unwrap().clap_args.time {
        process::exit(exit_code);
    }
    match SETTINGS.read().unwrap().subcommand_start.elapsed() {
        Ok(elapsed) => {
            println!("\nThe subcommand took {:?} to finish", elapsed);
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

    // Clone the process to the ctrlc thread (to be killed)
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
    if setting.clap_args.watch != 0.0 {
        while !*ctrlc_hit.read().unwrap() {
            clear_screen();
            exit_code = exec(arg_start, &mut subcommand_proc);
            thread::sleep(Duration::from_secs_f64(setting.clap_args.watch));
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

    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
    // writeln!(&mut stdout, "green text!");
    let stdout = BufReader::new(subcommand_proc.write().unwrap().stdout.take().unwrap());
    let stderr = BufReader::new(subcommand_proc.write().unwrap().stderr.take().unwrap());
    let stdout_bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let stderr_bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut stdout_bufwtr_boost = BufferedStandardStream::stdout(ColorChoice::Always);
    let mut stderr_bufwtr_boost = BufferedStandardStream::stderr(ColorChoice::Always);
    let is_nocolor = SETTINGS.read().unwrap().clap_args.nocolor;
    let is_boost = SETTINGS.read().unwrap().clap_args.boost;

    // Start to capture and color stdout
    let stdout_thread = thread::spawn(move || {
        stdout.lines().for_each(|line| {
            let ln = line.unwrap();
            if is_nocolor {
                print!("{}\n", ln);
                return;
            }
            if is_boost {
                // println!("boost!");
                color_std_boost(&mut stdout_bufwtr_boost, ln);
            } else {
                color_std(&stdout_bufwtr, ln);
            }
        });
    });

    // Start to capture and color stderr
    let stderr_thread = thread::spawn(move || {
        stderr.lines().for_each(|line| {
            let ln = line.unwrap();
            if is_nocolor {
                eprint!("{}\n", ln);
                return;
            }
            if is_boost {
                color_std_boost(&mut stderr_bufwtr_boost, ln);
            } else {
                color_std(&stderr_bufwtr, ln);
            }
        });
    });

    let status = subcommand_proc.write().unwrap().wait().unwrap();
    let exit_code = match status.code() {
        Some(code) => code,
        None => 0,
    };

    // Wait a longer here to make sure the subcommand exits
    // and to correctly capture the last word of the sub program
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    return exit_code;
}

fn color_std(bufwtr: &BufferWriter, ln: String) {
    let mut buffer = bufwtr.buffer();

    let mut main_string = vec![ColorString {
        text: ln,
        color: &Colors::Default,
    }];
    let main_string = colored_output(&mut main_string);

    for str in main_string.iter() {
        buffer.set_color(&get_color(str.color)).unwrap();
        write!(&mut buffer, "{}", str.text).unwrap();

        buffer.set_color(&get_color(&Colors::Default)).unwrap();
    }

    write!(&mut buffer, "\n").unwrap();
    bufwtr.print(&buffer).unwrap();
}

fn color_std_boost(bufwtr: &mut BufferedStandardStream, ln: String) {
    let mut main_string = vec![ColorString {
        text: ln,
        color: &Colors::Default,
    }];
    let main_string = colored_output(&mut main_string);

    for str in main_string.iter() {
        bufwtr.set_color(&get_color(str.color)).unwrap();
        write!(bufwtr, "{}", str.text).unwrap();
    }
    write!(bufwtr, "\n").unwrap();
}

fn get_color(color: &Colors) -> ColorSpec {
    let mut col = ColorSpec::new();
    match color {
        Colors::UnChanged => col // This one should be unreable
            .set_fg(Some(Color::Magenta))
            .set_intense(true)
            .set_underline(true),
        Colors::Default => col.set_fg(None),
        Colors::BDefault => col.set_bold(true).set_fg(None),
        Colors::Black => col.set_fg(Some(Color::Black)),
        Colors::Blue => col.set_fg(Some(Color::Blue)),
        Colors::Green => col.set_fg(Some(Color::Green)),
        Colors::Red => col.set_fg(Some(Color::Red)),
        Colors::Cyan => col.set_fg(Some(Color::Cyan)),
        Colors::Magenta => col.set_fg(Some(Color::Magenta)),
        Colors::Yellow => col.set_fg(Some(Color::Yellow)),
        Colors::White => col.set_fg(Some(Color::White)),
        Colors::BBlack => col.set_bold(true).set_fg(Some(Color::Ansi256(8))),
        Colors::BBlue => col.set_bold(true).set_fg(Some(Color::Blue)),
        Colors::BGreen => col.set_bold(true).set_fg(Some(Color::Green)),
        Colors::BRed => col.set_bold(true).set_fg(Some(Color::Red)),
        Colors::BCyan => col.set_bold(true).set_fg(Some(Color::Cyan)),
        Colors::BMagenta => col.set_bold(true).set_fg(Some(Color::Magenta)),
        Colors::BYellow => col.set_bold(true).set_fg(Some(Color::Yellow)),
        Colors::BWhite => col.set_bold(true).set_fg(Some(Color::White)),
        Colors::UDefault => col.set_underline(true).set_fg(None),
        Colors::UBlack => col.set_underline(true).set_fg(Some(Color::Black)),
        Colors::UBlue => col.set_underline(true).set_fg(Some(Color::Blue)),
        Colors::UGreen => col.set_underline(true).set_fg(Some(Color::Green)),
        Colors::URed => col.set_underline(true).set_fg(Some(Color::Red)),
        Colors::UCyan => col.set_underline(true).set_fg(Some(Color::Cyan)),
        Colors::UMagenta => col.set_underline(true).set_fg(Some(Color::Magenta)),
        Colors::UYellow => col.set_underline(true).set_fg(Some(Color::Yellow)),
        Colors::UWhite => col.set_underline(true).set_fg(Some(Color::White)),
        Colors::UBBlack => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Black)),
        Colors::UBBlue => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Blue)),
        Colors::UBGreen => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Green)),
        Colors::UBRed => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Red)),
        Colors::UBCyan => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Cyan)),
        Colors::UBMagenta => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Magenta)),
        Colors::UBYellow => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::Yellow)),
        Colors::UBWhite => col
            .set_underline(true)
            .set_bold(true)
            .set_fg(Some(Color::White)),
        Colors::UBDefault => col.set_underline(true).set_bold(true).set_fg(None),
        Colors::OnBlack => col.set_bg(Some(Color::Black)),
        Colors::OnBlue => col.set_bg(Some(Color::Blue)),
        Colors::OnGreen => col.set_bg(Some(Color::Green)),
        Colors::OnRed => col.set_bg(Some(Color::Red)),
        Colors::OnCyan => col.set_bg(Some(Color::Cyan)),
        Colors::OnMagenta => col.set_bg(Some(Color::Magenta)),
        Colors::OnYellow => col.set_bg(Some(Color::Yellow)),
        Colors::OnWhite => col.set_bg(Some(Color::White)),
        Colors::BlackOnBlack => col.set_bg(Some(Color::Black)).set_fg(Some(Color::Black)),
        Colors::BlackOnBlue => col.set_bg(Some(Color::Blue)).set_fg(Some(Color::Black)),
        Colors::BlackOnGreen => col.set_bg(Some(Color::Green)).set_fg(Some(Color::Black)),
        Colors::BlackOnRed => col.set_bg(Some(Color::Red)).set_fg(Some(Color::Black)),
        Colors::BlackOnCyan => col.set_bg(Some(Color::Cyan)).set_fg(Some(Color::Black)),
        Colors::BlackOnMagenta => col.set_bg(Some(Color::Magenta)).set_fg(Some(Color::Black)),
        Colors::BlackOnYellow => col.set_bg(Some(Color::Yellow)).set_fg(Some(Color::Black)),
        Colors::BlackOnWhite => col.set_bg(Some(Color::White)).set_fg(Some(Color::Black)),
        Colors::DDefault => col.set_dimmed(true).set_fg(None),
        Colors::DBlack => col.set_dimmed(true).set_fg(Some(Color::Black)),
        Colors::DBlue => col.set_dimmed(true).set_fg(Some(Color::Blue)),
        Colors::DGreen => col.set_dimmed(true).set_fg(Some(Color::Green)),
        Colors::DRed => col.set_dimmed(true).set_fg(Some(Color::Red)),
        Colors::DCyan => col.set_dimmed(true).set_fg(Some(Color::Cyan)),
        Colors::DMagenta => col.set_dimmed(true).set_fg(Some(Color::Magenta)),
        Colors::DYellow => col.set_dimmed(true).set_fg(Some(Color::Yellow)),
        Colors::DWhite => col.set_dimmed(true).set_fg(Some(Color::White)),
    };
    col
}

fn colored_output<'a>(main_string: &'a mut Vec<ColorString<'a>>) -> &'a Vec<ColorString<'a>> {
    let mut prev_color = &Colors::Default;
    for palette in SETTINGS.read().unwrap().palettes.iter() {
        let mut index = 0;
        // Instead of using a for loop, the size of main_string will grow so we have to use while loop
        // https://stackoverflow.com/questions/47338839
        while index < main_string.len() {
            // println!("i={} main_string.len()={}", index, main_string.len());

            if !main_string[index].color.eq(&Colors::Default) {
                index += 1;
                continue; // Ignore those already been colored
            }

            match palette
                .regexp
                .captures(main_string[index].text.as_str())
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
                        color: &Colors::Default,
                    });

                    // captures[0] -> Full match
                    // captures[1..] -> Group match
                    let mut last_start = captures.get(0).unwrap().start();
                    let full_match_end = captures.get(0).unwrap().end();
                    let mut is_full_match = false;
                    for (i, capture) in captures.iter().enumerate() {
                        if i == 0 {
                            let mut color = palette.colors[0];
                            if color == &Colors::UnChanged {
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

                        let mut color = palette.colors[0];
                        if color == &Colors::UnChanged {
                            color = prev_color;
                        }

                        colored_strings.push(ColorString {
                            text: String::from_str(&str[last_start..start]).unwrap(),
                            color: &color,
                        });
                        prev_color = color;

                        if i < palette.colors.len() {
                            color = palette.colors[i];
                            if color == &Colors::UnChanged {
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
                        color: &Colors::Default,
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
    main_string.retain(|color_string| color_string.text != "");
    // println!("main_string={:?}", main_string);
    main_string
}
