use clap::Clap;
use drosera::tarpit;
use std::net::SocketAddr;

/// Tarpit SSH server.
#[derive(Clap, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Opts {
    /// The socket address to bind to
    #[clap(short, long, default_value = "127.0.0.1:22")]
    socket_addr: SocketAddr,
    /// The maximum number of connections maintained at once
    #[clap(short, long, default_value = "1024")]
    max_connections: u32,
    /// Approximately wait this long before sending more data (in milliseconds)
    #[clap(short, long, default_value = "10000")]
    delay: u32,
}

fn main() {
    let opts = Opts::parse();
    println!("{:#?}", opts);
    start_server(opts);
}

fn start_server(opts: Opts) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let options = tarpit::TarpitServerOptions {
                socket_addr: opts.socket_addr,
                max_connections: opts.max_connections,
                delay: opts.delay,
            };
            let _ = tarpit::start_server(options).await;
        });
}
