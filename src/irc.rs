use std::collections::HashMap;
use std::net::SocketAddr;

use commands::Command;
use commands::parse_command;
use responses::ErrorResponse;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use user::User;

pub mod commands;
pub mod responses;
pub mod user;

#[derive(Debug)]
pub struct IRCServer {
    listener: TcpListener,
    users: HashMap<SocketAddr, User>,
}

enum SocketMessage {
    Text(String),
    Disconnect,
}

impl IRCServer {
    async fn disconnect_user(&mut self, addr: SocketAddr) {
        println!("Disconnected");
        self.users.remove(&addr);
    }

    async fn exec_command(&mut self, cmd: Command, addr: &SocketAddr) -> Result<(), ErrorResponse> {
        match cmd {
            Command::NICK(nickname) => {
                if !self.users.contains_key(addr) {
                    let user = User::default().with_nickname(nickname);
                    self.users.insert(*addr, user);
                } else {
                    let user = self.users.get_mut(addr).unwrap();
                    user.set_nickname(nickname);
                    if !user.username.is_empty() {
                        user.registered = true;
                    }
                }

                Ok(())
            }
            Command::USER(username, mode, unused, realname) => {
                if let Some(user) = self.users.get_mut(addr) {
                    if user.registered {
                        return Err(ErrorResponse::AlreadyRegistered);
                    }
                    user.set_username(username);
                    user.set_realname(realname);
                } else {
                    let user = User::default().with_user(username, realname);
                    self.users.insert(*addr, user);
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

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
                        let _ = tx.send(SocketMessage::Disconnect).await;
                        break;
                    }
                    Ok(_) => {
                        let _ = tx
                            .send(SocketMessage::Text(buffer.trim().to_string()))
                            .await;
                    }
                    Err(e) => {
                        eprintln!("{e}");
                    }
                }
            }
        });

        while let Some(message) = rx.recv().await {
            match &message {
                SocketMessage::Text(msg) => match parse_command(&msg) {
                    Ok((_prefix, cmd)) => {
                        println!("Found command: {cmd:?}");
                        let _ = self.exec_command(cmd, &_addr).await;
                        println!("{0:?}", self.users);
                        let _ = write.write_all(b"walpurgisnact\n").await;
                    }
                    Err(response_code) => {
                        eprintln!("Err: {response_code:?}");
                        let _ = write.write_u8(response_code as u8).await;
                    }
                },
                SocketMessage::Disconnect => {
                    self.disconnect_user(_addr).await;
                    break;
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
