use crate::cli::cli::{exec, Colours, Palette};
use clap::{App, AppSettings, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct CMD {}

impl CMD {
    pub fn new() -> App<'static, 'static> {
        App::new("ping")
            .args(&[
                Arg::with_name("url").help("URL destination").required(true),
                Arg::with_name("4").short("4").help("Use IPv4 only."),
                Arg::with_name("6").short("6").help("Use IPv6 only."),
                Arg::with_name("a").short("a").help("Audible ping."),
                Arg::with_name("A").short("A").help("Adaptive ping. Interpacket interval adapts to round-trip time"),
                Arg::with_name("b").short("b").help("Allow pinging a broadcast address."),
                Arg::with_name("B").short("B").help("Do not allow ping to change source address of probes. The address is bound to one selected when ping starts."),
                Arg::with_name("count").short("c").takes_value(true).help("Stop after sending count ECHO_REQUEST packets. With deadline option, ping waits for count ECHO_REPLY packets, until the timeout expires."),
                Arg::with_name("d").short("d").help("Set the SO_DEBUG option on the socket being used. Essentially, this socket option is not used by Linux kernel."),
                Arg::with_name("D").short("D").help("Print timestamp (unix time + microseconds as in gettimeofday) before each line."),
                Arg::with_name("f").short("f").help("Flood ping. For every ECHO_REQUEST sent a period “.” is printed"),
                Arg::with_name("flow").short("F").takes_value(true).help("IPv6 only. Allocate and set 20 bit flow label (in hex) on echo request packets. If value is zero, kernel allocates random flow label."),
                Arg::with_name("h").short("h").help("Show help."),
                Arg::with_name("interval").short("i").takes_value(true).help("Wait interval seconds between sending each packet."),
                Arg::with_name("interface").short("I").takes_value(true).help("interface is either an address, an interface name or a VRF name."),
                Arg::with_name("preload").short("l").takes_value(true).help("If preload is specified, ping sends that many packets not waiting for reply. Only the super-user may select preload more than 3."),
                Arg::with_name("L").short("L").help("Suppress loopback of multicast packets. This flag only applies if the ping destination is a multicast address."),
                Arg::with_name("mark").short("m").takes_value(true).help("use mark to tag the packets going out. This is useful for variety of reasons within the kernel such as using policy routing to select specific outbound processing."),
                Arg::with_name("pmtudisc_opt").short("M").takes_value(true).help("Select Path MTU Discovery strategy."),
                Arg::with_name("nodeinfo_option").short("N").takes_value(true).help("IPv6 only. Send ICMPv6 Node Information Queries (RFC4620), instead of Echo Request. CAP_NET_RAW capability is required."),
                Arg::with_name("n").short("n").help("Numeric output only."),
                Arg::with_name("O").short("O").help("Report outstanding ICMP ECHO reply before sending next packet."),
                Arg::with_name("pattern").short("p").help("You may specify up to 16 “pad” bytes to fill out the packet you send."),
                Arg::with_name("q").short("q").help("Quiet output."),
                Arg::with_name("tos").short("Q").takes_value(true).help("Set Quality of Service -related bits in ICMP datagrams.  tos can be decimal (ping only) or hex number."),
                Arg::with_name("r").short("r").help("Bypass the normal routing tables and send directly to a host on an attached interface."),
                Arg::with_name("R").short("R").help("Record route."),
                Arg::with_name("packetsize").short("s").takes_value(true).help("Specifies the number of data bytes to be sent."),
                Arg::with_name("sndbuf").short("S").takes_value(true).help("Set socket sndbuf."),
                Arg::with_name("ttl").short("t").takes_value(true).help("Set the IP Time to Live."),
                Arg::with_name("timestamp").short("T").takes_value(true).help("Set special IP timestamp options."),
                Arg::with_name("U").short("U").help("Print full user-to-user latency (the old behaviour)."),
                Arg::with_name("v").short("v").help("Verbose output."),
                Arg::with_name("V").short("V").help("Show version and exit."),
                Arg::with_name("deadline").short("w").takes_value(true).help("Specify a timeout, in seconds, before ping exits regardless of how many packets have been sent or received."),
                Arg::with_name("timeout").short("W").takes_value(true).help("Time to wait for a response, in seconds."),
            ])
            .setting(AppSettings::ArgRequiredElseHelp)
            .about("ping")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        exec(CMD::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // IP
            Palette {
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap(),
                colours: vec![&Colours::BBlue],
            },
            // ipv6 number
            Palette {
                regexp: Regex::new(r#"(([0-9a-fA-F]{1,4})?\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colours: vec![&Colours::Magenta],
            },
            // icmp_seq=
            Palette {
                regexp: Regex::new(r#"icmp_seq=(\d+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // ttl=
            Palette {
                regexp: Regex::new(r#"ttl=(\d+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // name
            Palette {
                regexp: Regex::new(r#"(?:[fF]rom|PING)\s(\S+)\s"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Blue],
            },
            // DUP
            Palette {
                regexp: Regex::new(r#"DUP\!"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // OK
            Palette {
                regexp: Regex::new(r#" 0(\.0)?% packet loss"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // Errors
            Palette {
                regexp: Regex::new(r#"(Destination Host Unreachable|100(\.0)?% packet loss)"#)
                    .unwrap(),
                colours: vec![&Colours::Red],
            },
            // unknown host
            Palette {
                regexp: Regex::new(r#".+unknown\shost\s(.+)"#).unwrap(),
                colours: vec![&Colours::Red, &Colours::BRed],
            },
            // statistics header
            Palette {
                regexp: Regex::new(r#"--- (\S+) ping statistics ---"#).unwrap(),
                colours: vec![&Colours::BDefault, &Colours::BBlue],
            },
            // last line min/avg/max/mdev
            Palette {
                regexp: Regex::new(r#"rtt (min)/(avg)/(max)/(mdev)"#).unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BYellow,
                    &Colours::BBlue,
                    &Colours::BRed,
                    &Colours::BMagenta,
                ],
            },
            // last line values
            Palette {
                regexp: Regex::new(r#"\=\s([0-9\.]+)\/([0-9\.]+)\/([0-9\.]+)\/([0-9\.]+)"#)
                    .unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BYellow,
                    &Colours::BBlue,
                    &Colours::BRed,
                    &Colours::BMagenta,
                ],
            },
            // these are good for nping
            Palette {
                regexp: Regex::new(r#"SENT|RCVD"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // nping
            Palette {
                regexp: Regex::new(r#"unreachable"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // time
            Palette {
                regexp: Regex::new(r#"([0-9\.]+)\s?ms"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
        ]
    }
}
