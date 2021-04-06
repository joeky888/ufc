use clap::{App, AppSettings};
mod cli;
use cli::{docker::Docker, ping::Ping};

use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct ColorString<'a> {
    text: String,
    color: &'a Colours,
}

struct Palette<'a> {
    regexp: Regex,
    colours: Vec<&'a Colours>,
}

#[derive(Debug, PartialEq)]
enum Colours {
    Default,
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
fn main() {
    let mut main_string = vec![ColorString {
        text: "64 bytes from 8.8.8.8: icmp_seq=1 ttl=116 time=4.05 ms".to_string(),
        color: &Colours::Default,
    }];

    let palettes = vec![
        // IP
        Palette {
            regexp: Regex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap(),
            colours: vec![&Colours::BoldBlue],
        },
        // icmp_seq=
        Palette {
            regexp: Regex::new(r"icmp_seq=(\d+)").unwrap(),
            colours: vec![&Colours::Default, &Colours::Magenta],
        },
        // ttl=
        Palette {
            regexp: Regex::new(r"ttl=(\d+)").unwrap(),
            colours: vec![&Colours::Default, &Colours::Magenta],
        },
        // time
        Palette {
            regexp: Regex::new(r"([0-9\.]+)\s?ms").unwrap(),
            colours: vec![&Colours::Green, &Colours::BoldGreen],
        },
    ];

    for palette in palettes.iter() {
        for i in 0..main_string.len() {
            if !main_string[i].color.eq(&Colours::Default) {
                continue; // Ignore those already been colored
            }

            match palette
                .regexp
                .captures(main_string[i].text.clone().as_str())
            {
                Some(captures) => {
                    println!("!!!!i={} {:?}!!!", i, captures);

                    let str = main_string[i].text.as_str();
                    // println!("!!!!{:?}!!!", str);
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
                        let before_start = new_start;
                        let before_end = captures.get(i).unwrap().start();
                        let start = captures.get(i).unwrap().start();
                        let end = captures.get(i).unwrap().end();
                        let after_start = captures.get(i).unwrap().end();
                        let after_end = new_end;

                        colored_strings.pop(); // Remove the last one because we have to split it into 2 elements
                        colored_strings.push(ColorString {
                            text: String::from_str(&str[before_start..before_end]).unwrap(),
                            color: palette.colours[0],
                        });
                        colored_strings.push(ColorString {
                            text: String::from_str(&str[start..end]).unwrap(),
                            color: palette.colours[i],
                        });
                        // colored_strings.push(ColorString {
                        //     text: String::from_str(&str[after_start..after_end]).unwrap(),
                        //     color: palette.colours[0],
                        // });

                        if i == captures.len() - 1 {
                            // Push the last one (The rest of the string) back when the for loop ends
                            // Because the for loop ends here, so we don't need to split the rest of the string anymore
                            colored_strings.push(ColorString {
                                text: String::from_str(&str[after_start..after_end]).unwrap(),
                                color: palette.colours[0],
                            });
                        }
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

                    main_string[i].text = String::new();
                    main_string.remove(i);
                    main_string.splice((i)..(i), colored_strings);
                }
                None => {}
            };
        }
    }

    // Remove empty strings
    main_string.retain(|color_string| color_string.text != "");
    println!("{:?}", main_string);

    let app = App::new("ufc")
        .version("v0.0.1")
        .about("Ultimate Fantastic CLI")
        .author("Joeky <https://github.com/joeky888>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::ColorAlways)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DisableHelpFlags)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .global_setting(AppSettings::AllowExternalSubcommands)
        .global_setting(AppSettings::TrailingValues)
        .subcommands(vec![Docker::new(), Ping::new()])
        .get_matches();

    match app.subcommand() {
        Some(("docker", args)) => Docker::parse(args),
        Some(("ping", args)) => Ping::parse(args),
        None => println!("No subcommand was used"),
        _ => {}
    }
}
