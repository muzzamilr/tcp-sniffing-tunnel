use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub async fn tcp_sniffer(from: &str, to: &str) -> Result<()> {
    let listener = TcpListener::bind(from).await?;
    while let Ok((client_req, _)) = listener.accept().await {
        tokio::spawn(handle_request(client_req, to.to_string()));
    }
    Ok(())
}

async fn handle_request(mut client_req: TcpStream, to: String) {
    let mut server = TcpStream::connect(to).await.unwrap();
    let (mut client_reader, mut client_writer) = client_req.split();
    let (mut remote_reader, mut remote_writer) = server.split();

    let mut buffer = [0u8; 1024];
    loop {
        match client_reader.read(&mut buffer).await {
            Ok(0) => break, // End of stream
            Ok(n) => {
                let data = &buffer[..n];
                println!("Received: {:?}", data);
                remote_writer.write_all(data).await.unwrap();
            }
            Err(_) => break,
        }
    }
}
