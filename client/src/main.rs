use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buf = [0; 1024];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        let msg = String::from_utf8_lossy(&buf[..n]);
        if msg.starts_with("START") {
            println!("Experiment started!");
            // Send guess
            let guess = 42; // Example guess
            stream.write_all(format!("GUESS {}\n", guess).as_bytes()).await?;
        }
    }

    Ok(())
}