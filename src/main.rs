use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::{Duration, Instant};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 22);
    let listener = TcpListener::bind(&socket).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let _ = process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) -> io::Result<()> {
    let peer = socket.peer_addr()?;
    println!("{:?} connected", peer);
    let connected = Instant::now();
    let sent_bytes = keep_busy(socket).await;
    let elapsed = connected.elapsed();
    println!(
        "{:?} disconnected, was connected for {}ms, received {} bytes",
        peer,
        elapsed.as_millis(),
        sent_bytes
    );
    Ok(())
}

async fn keep_busy(mut socket: TcpStream) -> u64 {
    let mut sent_bytes: u64 = 0;
    loop {
        let data = format!("{}\r\n", rand_string_clrf());

        if socket.write_all(data.as_bytes()).await.is_ok() {
            sent_bytes += data.len() as u64;
        } else {
            return sent_bytes;
        }

        sleep(Duration::from_millis(10000)).await;
    }
}

fn rand_string_clrf() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}
