use clap::{Arg, Command, crate_name, crate_version, value_parser};
use drosera::tarpit;
use std::net::SocketAddr;

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .long_version(crate_version!())
        .arg_required_else_help(true)
        .about("Tarpit SSH server")
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
        .get_matches();

    let options = tarpit::TarpitServerOptions {
        socket_addr: *matches.get_one::<SocketAddr>("socket_addr").unwrap(),
        max_connections: *matches.get_one::<u32>("max_connections").unwrap(),
        delay: *matches.get_one::<u32>("delay").unwrap(),
    };
    println!("{:#?}", options);
    start_server(options);
}

fn start_server(options: tarpit::TarpitServerOptions) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let _ = tarpit::start_server(options).await;
        });
}
