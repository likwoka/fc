//! fcweb
//!
//! A web app for temperature conversions.
use fcweb_lib;
use std::{env, net::{self, Ipv4Addr, SocketAddrV4}};

const HELP_MSG: &str = r"fcweb -- a web app for temperature conversions.

Usage:
fcweb                    # default to 127.0.0.1:8080
fcweb 192.168.12.1:8881  # bind webapp to IP address and port
fcweb -c conf.toml       # use configuration file
fcweb -h                 # print this help message";

enum CmdLineMode {
    DefaultSocketAddr,
    CustomSocketAddr(net::SocketAddrV4),
    PrintMsg(String),
}

fn parse_cmdline(args: &Vec<String>) -> CmdLineMode {
    match args.len() {
        1 => CmdLineMode::DefaultSocketAddr,
        2 if args[1] == "-h" => CmdLineMode::PrintMsg(HELP_MSG.into()),
        2 => {
            let addr = args[1].parse();
            match addr {
                Ok(value) => CmdLineMode::CustomSocketAddr(value),
                Err(_) => CmdLineMode::PrintMsg(
                    ["Error: Unable to parse", &args[1], "to socket address"].join(" "),
                ),
            }
        }
        _ => CmdLineMode::PrintMsg(HELP_MSG.into()),
    }
}

fn main() {
    let mode = parse_cmdline(&env::args().collect());
    let addr_n_port = match mode {
        CmdLineMode::PrintMsg(msg) => {
            println!("{}", msg);
            std::process::exit(1);
        },
        CmdLineMode::CustomSocketAddr(addr) => addr,
        CmdLineMode::DefaultSocketAddr => SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)
    };
    println!("fcweb listening on {}", addr_n_port);
    fcweb_lib::webmain(addr_n_port).unwrap();
}
