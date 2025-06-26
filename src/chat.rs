use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    stream
        .write_all(b"Hello new user, type something...\n")
        .await?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader.read_line(&mut line).await?;

    // into_inner takes out the original TcpStream and into_split splits reader and writer from the
    // stream
    let (_reader, mut writer) = reader.into_inner().into_split();
    writer.write_all(b"You sent: ").await?;
    writer.write_all(line.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:2373").await?;
    log::info!("Listening on 0.0.0.0:2373");

    loop {
        let (tcpstream, _remote_peer_addr) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(tcpstream).await {
                log::error!("Error handling connection: {}", e);
            }
        });
    }
}
