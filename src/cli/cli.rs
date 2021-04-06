use std::{
    env,
    io::{BufRead, BufReader, Write},
    process::{self, Command, Stdio},
    sync::{Arc, RwLock},
    thread,
};

use regex::Regex;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub enum Colours {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    BoldBlack,
    BoldBlue,
    BoldGreen,
    BoldRed,
    BoldCyan,
    BoldMagenta,
    BoldYellow,
    BoldWhite,
}

pub struct Palette {
    regexp: Regex,
    colours: Vec<Colours>,
}

pub fn exec() {
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

    // let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let palettes = vec![
        // IP
        Palette {
            regexp: Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap(),
            colours: vec![Colours::Blue],
        },
        // icmp_seq=
        Palette {
            regexp: Regex::new(r"icmp_seq=(\d+)").unwrap(),
            colours: vec![Colours::Red],
        },
    ];

    // Start to capture stdout
    let stdout_thread = thread::spawn(move || {
        stdout.lines().for_each(|line| {
            let bufwtr = BufferWriter::stdout(ColorChoice::Always);
            let mut buffer_writer = bufwtr.buffer();
            let ln = line.unwrap();
            let index: usize = 0;
            let mut buf = bufwtr.buffer();
            for palette in palettes.iter() {
                match palette.regexp.captures(ln.as_str()) {
                    Some(caps) => {
                        // println!("!!!!{:?}!!!", caps.get(1).unwrap().start());
                        for (i, cap) in caps.iter().enumerate() {
                            match cap {
                                Some(c) => {
                                    // buf.set_color(&get_color(&palette.colours[i])).unwrap();
                                    println!("!!!!!{:?}!!!!!", c);
                                    println!("!!!!!{:?}!!!!!", c.start());
                                    println!("!!!!!{:?}!!!!!", c.end());
                                    let sub_str: String =
                                        ln.chars().skip(index).take(c.start() - index).collect();
                                    // println!("!!!!!{:?}!!!!!", sub_str);
                                    // sub_str.as_str()
                                    write!(&mut buf, "{}", sub_str).unwrap();
                                }
                                None => continue,
                            }
                        }
                    }
                    None => continue,
                }
            }

            // buffer1
            //     .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            //     .unwrap();
            // buffer2
            //     .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
            //     .unwrap();
            // write!(&mut buffer1, "green text!").unwrap();
            // write!(&mut buffer2, "blue text!").unwrap();

            buffer_writer.write(&buf.as_slice().to_vec()).unwrap();
            // buffer_writer.write(&buffer2.as_slice().to_vec()).unwrap();
            bufwtr.print(&buffer_writer).unwrap();
            println!("{}", ln)
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
        // log::debug!("ctrlc received!");
        print!("ctrlc received!");
        // Ignore kill() error, because the program exits anyway
        match child_clone.write().unwrap().kill() {
            Err(_) => (),
            Ok(_) => (),
        }
    })
    .unwrap();

    let status = child.write().unwrap().wait().unwrap();
    let mut exit_code = 0;
    match status.code() {
        Some(code) => exit_code = code,
        None => {
            exit_code = 0;
        }
    }

    // For some reason, we have to wait a longer here to make sure the sub program exits
    // And to correctly capture the last word of the sub program
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    process::exit(exit_code);
}

fn get_color(color: &Colours) -> ColorSpec {
    let mut col = ColorSpec::new();
    match color {
        Colours::Black => col.set_fg(Some(Color::Black)),
        Colours::Blue => col.set_fg(Some(Color::Blue)),
        Colours::Green => col.set_fg(Some(Color::Green)),
        Colours::Red => col.set_fg(Some(Color::Red)),
        Colours::Cyan => col.set_fg(Some(Color::Cyan)),
        Colours::Magenta => col.set_fg(Some(Color::Magenta)),
        Colours::Yellow => col.set_fg(Some(Color::Yellow)),
        Colours::White => col.set_fg(Some(Color::White)),
        Colours::BoldBlack => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldBlue => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldGreen => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldRed => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldCyan => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldMagenta => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldYellow => col.set_bold(true).set_fg(Some(Color::Black)),
        Colours::BoldWhite => col.set_bold(true).set_fg(Some(Color::Black)),
    };
    col
}
