use crate::cli::cli::{pre_exec, Colours, Palette};
use clap::{App, AppSettings, Arg, ArgMatches};
use fancy_regex::Regex;

pub struct Cmd {}

impl Cmd {
    pub fn new() -> App<'static> {
        App::new("ping")
            .args(&[
                Arg::new("url").about("URL destination").required(true),
                Arg::new("4").short('4').about("Use IPv4 only."),
                Arg::new("6").short('6').about("Use IPv6 only."),
                Arg::new("a").short('a').about("Audible ping."),
                Arg::new("A").short('A').about("Adaptive ping. Interpacket interval adapts to round-trip time"),
                Arg::new("b").short('b').about("Allow pinging a broadcast address."),
                Arg::new("B").short('B').about("Do not allow ping to change source address of probes. The address is bound to one selected when ping starts."),
                Arg::new("count").short('c').takes_value(true).about("Stop after sending count ECHO_REQUEST packets. With deadline option, ping waits for count ECHO_REPLY packets, until the timeout expires."),
                Arg::new("d").short('d').about("Set the SO_DEBUG option on the socket being used. Essentially, this socket option is not used by Linux kernel."),
                Arg::new("D").short('D').about("Print timestamp (unix time + microseconds as in gettimeofday) before each line."),
                Arg::new("f").short('f').about("Flood ping. For every ECHO_REQUEST sent a period “.” is printed"),
                Arg::new("flow").short('F').takes_value(true).about("IPv6 only. Allocate and set 20 bit flow label (in hex) on echo request packets. If value is zero, kernel allocates random flow label."),
                Arg::new("h").short('h').about("Show help."),
                Arg::new("interval").short('i').takes_value(true).about("Wait interval seconds between sending each packet."),
                Arg::new("interface").short('I').takes_value(true).about("interface is either an address, an interface name or a VRF name."),
                Arg::new("preload").short('l').takes_value(true).about("If preload is specified, ping sends that many packets not waiting for reply. Only the super-user may select preload more than 3."),
                Arg::new("L").short('L').about("Suppress loopback of multicast packets. This flag only applies if the ping destination is a multicast address."),
                Arg::new("mark").short('m').takes_value(true).about("use mark to tag the packets going out. This is useful for variety of reasons within the kernel such as using policy routing to select specific outbound processing."),
                Arg::new("pmtudisc_opt").short('M').takes_value(true).about("Select Path MTU Discovery strategy."),
                Arg::new("nodeinfo_option").short('N').takes_value(true).about("IPv6 only. Send ICMPv6 Node Information Queries (RFC4620), instead of Echo Request. CAP_NET_RAW capability is required."),
                Arg::new("n").short('n').about("Numeric output only."),
                Arg::new("O").short('O').about("Report outstanding ICMP ECHO reply before sending next packet."),
                Arg::new("pattern").short('p').about("You may specify up to 16 “pad” bytes to fill out the packet you send."),
                Arg::new("q").short('q').about("Quiet output."),
                Arg::new("tos").short('Q').takes_value(true).about("Set Quality of Service -related bits in ICMP datagrams.  tos can be decimal (ping only) or hex number."),
                Arg::new("r").short('r').about("Bypass the normal routing tables and send directly to a host on an attached interface."),
                Arg::new("R").short('R').about("Record route."),
                Arg::new("packetsize").short('s').takes_value(true).about("Specifies the number of data bytes to be sent."),
                Arg::new("sndbuf").short('S').takes_value(true).about("Set socket sndbuf."),
                Arg::new("ttl").short('t').takes_value(true).about("Set the IP Time to Live."),
                Arg::new("timestamp").short('T').takes_value(true).about("Set special IP timestamp options."),
                Arg::new("U").short('U').about("Print full user-to-user latency (the old behaviour)."),
                Arg::new("v").short('v').about("Verbose output."),
                Arg::new("V").short('V').about("Show version and exit."),
                Arg::new("deadline").short('w').takes_value(true).about("Specify a timeout, in seconds, before ping exits regardless of how many packets have been sent or received."),
                Arg::new("timeout").short('W').takes_value(true).about("Time to wait for a response, in seconds."),
            ])
            .setting(AppSettings::ArgRequiredElseHelp)
            .about("ping")
    }

    pub fn parse(_app: &ArgMatches) {
        // print!("{:?}", app);
        pre_exec(Cmd::palette());
    }

    fn palette() -> Vec<Palette<'static>> {
        vec![
            // nping
            Palette {
                regexp: Regex::new(r#"unreachable"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // these are good for nping
            Palette {
                regexp: Regex::new(r#"SENT|RCVD"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // last line values
            Palette {
                regexp: Regex::new(r#"\=\s([0-9\.]+)/([0-9\.]+)/([0-9\.]+)/([0-9\.]+)"#).unwrap(),
                colours: vec![
                    &Colours::Default,
                    &Colours::BYellow,
                    &Colours::BBlue,
                    &Colours::BRed,
                    &Colours::BMagenta,
                ],
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
            // statistics header
            Palette {
                regexp: Regex::new(r#"--- (\S+) ping statistics ---"#).unwrap(),
                colours: vec![&Colours::BDefault, &Colours::BBlue],
            },
            // unknown host
            Palette {
                regexp: Regex::new(r#".+unknown\shost\s(.+)"#).unwrap(),
                colours: vec![&Colours::Red, &Colours::BRed],
            },
            // Errors
            Palette {
                regexp: Regex::new(r#"(Destination Host Unreachable|100(\.0)?% packet loss)"#)
                    .unwrap(),
                colours: vec![&Colours::Red],
            },
            // OK
            Palette {
                regexp: Regex::new(r#" 0(\.0)?% packet loss"#).unwrap(),
                colours: vec![&Colours::Green],
            },
            // DUP
            Palette {
                regexp: Regex::new(r#"DUP\!"#).unwrap(),
                colours: vec![&Colours::Red],
            },
            // time
            Palette {
                regexp: Regex::new(r#"([0-9\.]+)?\s?ms"#).unwrap(),
                colours: vec![&Colours::Green, &Colours::BGreen],
            },
            // name
            Palette {
                regexp: Regex::new(r#"(?:[fF]rom|PING)\s(\S+)\s"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Blue],
            },
            // ttl=
            Palette {
                regexp: Regex::new(r#"ttl=(\d+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // icmp_seq=
            Palette {
                regexp: Regex::new(r#"icmp_seq=(\d+)"#).unwrap(),
                colours: vec![&Colours::Default, &Colours::Magenta],
            },
            // ipv6 number
            Palette {
                regexp: Regex::new(r#"(([0-9a-fA-F]{1,4})?\:\:?[0-9a-fA-F]{1,4})+"#).unwrap(),
                colours: vec![&Colours::Magenta],
            },
            // IP
            Palette {
                regexp: Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}"#).unwrap(),
                colours: vec![&Colours::BBlue],
            },
        ]
    }
}
