use std::collections::HashMap;
use std::iter::Map;
use std::net::SocketAddr;

use commands::Command;
use commands::execute_command;
use commands::parse_command;
use responses::ErrorResponse;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub mod commands;
pub mod responses;

#[derive(Default, Debug)]
pub struct User {
    nickname: String,
    password: String,
}

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    users: HashMap<SocketAddr, User>,
}

impl IRCServer {
    async fn process_socket(&mut self, socket: TcpStream) {
        let _addr = socket.peer_addr().unwrap();
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
                    Err(e) => {
                        eprintln!("{e}");
                    }
                }
            }
        });

        while let Some(message) = rx.recv().await {
            match parse_command(&message) {
                Ok((_prefix, cmd)) => {
                    println!("Found command: {cmd:?}");
                    let _ = execute_command(self, cmd);
                    let _ = write.write_all(b"walpurgisnact\n").await;
                }
                Err(response_code) => {
                    eprintln!("Err: {response_code:?}");
                    let _ = write.write_u8(response_code as u8).await;
                }
            }
        }
    }

    pub async fn new(address: &str) -> Self {
        Self {
            listener: TcpListener::bind(address).await.unwrap(),
            users: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        println!("Running IRC server");
        loop {
            let (socket, _) = self.listener.accept().await.unwrap();
            self.process_socket(socket).await;
        }
    }
}
