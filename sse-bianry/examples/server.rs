use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on 0.0.0.0:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);

        tokio::spawn(async move {
            let mut buf = BytesMut::with_capacity(1024);

            loop {
                let mut temp_buf = [0u8; 1024];
                let n = match socket.read(&mut temp_buf).await {
                    Ok(0) => {
                        println!("Connection closed");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read: {}", e);
                        return;
                    }
                };

                buf.extend_from_slice(&temp_buf[..n]);

                while buf.len() >= 4 {
                    println!("Received message: {:?}", buf);
                }
            }
        });
    }
}
