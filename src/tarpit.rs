use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    error::Error,
    io,
    net::SocketAddr,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    time::sleep,
};

#[derive(Clone, Debug)]
pub struct TarpitServerOptions {
    pub socket_addr: SocketAddr,
    pub max_connections: u32,
    pub delay: u32,
}

pub async fn start_server(options: TarpitServerOptions) -> Result<(), Box<dyn Error>> {
    let socket = options.socket_addr;
    let delay = options.delay;
    let max_connections = options.max_connections;
    let current_connections = Arc::new(AtomicU32::new(0));
    let listener = TcpListener::bind(&socket).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        current_connections.fetch_add(1, Ordering::Relaxed);
        if current_connections.load(Ordering::Relaxed) >= max_connections {
            println!("Max connections reached, refusing connection");
            current_connections.fetch_sub(1, Ordering::Relaxed);
            drop(socket);
            continue;
        }

        let cloned_connections = current_connections.clone();
        tokio::spawn(async move {
            let _ = process(socket, delay).await;
            cloned_connections.fetch_sub(1, Ordering::Relaxed);
        });
    }
}

async fn process(socket: TcpStream, delay: u32) -> io::Result<()> {
    let peer = socket.peer_addr()?;
    println!("{:?} connected", peer);
    let connected = Instant::now();
    let sent_bytes = keep_busy(socket, delay).await;
    let elapsed = connected.elapsed();
    println!(
        "{:?} disconnected, was connected for {}ms, received {} bytes",
        peer,
        elapsed.as_millis(),
        sent_bytes
    );
    Ok(())
}

async fn keep_busy(mut socket: TcpStream, delay: u32) -> u64 {
    let mut sent_bytes: u64 = 0;
    loop {
        let data = format!("{}\r\n", rand_string(30));

        if socket.write_all(data.as_bytes()).await.is_ok() {
            sent_bytes += data.len() as u64;
        } else {
            return sent_bytes;
        }
        let range = (f64::from(delay) * 0.7) as u64..(f64::from(delay) * 1.3) as u64;
        let time = thread_rng().gen_range(range);
        sleep(Duration::from_millis(time)).await;
    }
}

fn rand_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(length)
        .collect()
}
