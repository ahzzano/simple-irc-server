use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process_socket(socket).await;
    }
}

async fn process_socket(mut socket: TcpStream) {}
