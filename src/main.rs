use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub mod irc;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process_socket(socket).await;
    }
}

async fn process_socket(mut socket: TcpStream) {
    let mut buffer = String::new();
    let peer_address = socket.peer_addr().unwrap();
    let _new_user = irc::User::default();

    socket.read_to_string(&mut buffer).await.unwrap();
    println!("{peer_address:?}: {buffer}");
}
