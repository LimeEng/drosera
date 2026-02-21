use crate::tarpit;
use clap::{Arg, ArgMatches, Command, value_parser};
use std::net::SocketAddr;
use tokio::runtime::Runtime;

pub fn command() -> Command {
    Command::new("ssh")
        .about("Starts a SSH tarpit server")
        .arg(
            Arg::new("socket_addr")
                .value_parser(value_parser!(SocketAddr))
                .default_value("127.0.0.1:22")
                .short('s')
                .long("socket_addr")
                .help("The socket address to bind to"),
        )
        .arg(
            Arg::new("max_connections")
                .value_parser(value_parser!(u32))
                .default_value("1024")
                .short('m')
                .long("max_connections")
                .help("The maximum number of connections maintained at once"),
        )
        .arg(
            Arg::new("delay")
                .value_parser(value_parser!(u32))
                .default_value("10000")
                .short('d')
                .long("delay")
                .help("Approximately wait this long before sending more data (in milliseconds)"),
        )
}

pub fn execute(matches: &ArgMatches) {
    let socket_addr = *matches.get_one("socket_addr").unwrap();
    let max_connections = *matches.get_one("max_connections").unwrap();
    let delay = *matches.get_one("delay").unwrap();
    let options = tarpit::TarpitServerOptions {
        socket_addr,
        max_connections,
        delay,
    };
    start_server(options);
}

fn start_server(options: tarpit::TarpitServerOptions) {
    println!("{options:#?}");
    let rt = Runtime::new().expect("failed to create runtime");
    if let Err(e) = rt.block_on(tarpit::start_server(options)) {
        eprintln!("server error: {e}");
    }
}
