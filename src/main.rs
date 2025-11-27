use irc::IRCServer;

pub mod irc;

#[tokio::main]
async fn main() {
    let mut irc = IRCServer::new("127.0.0.1:25565").await;
    irc.run().await;
}
