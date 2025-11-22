use crate::irc::commands::parse_command;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub mod irc;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process_socket(socket).await;
    }
}

async fn process_socket(socket: TcpStream) {
    let (read, mut write) = tokio::io::split(socket);
    let mut reader = BufReader::new(read);

    let (tx, mut rx) = mpsc::channel(128);

    tokio::spawn(async move {
        loop {
            let mut buffer = String::new();

            match reader.read_line(&mut buffer).await {
                Ok(0) => {
                    println!("Reader disconnected");
                    break;
                }
                Ok(_) => {
                    let _ = tx.send(buffer.trim().to_string()).await;
                    println!("Got message");
                }
                Err(_) => todo!(),
            }
        }
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {message}");
        let _cmd = parse_command(&message);

        let _ = write.write_all(b"walpurgisnact\n").await;
    }
}
