use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Address to bind the server to
    #[clap(short, long, default_value = "127.0.0.1:8080")]
    address: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut stream = TcpStream::connect(&args.address)?;
    let mut buf = [0; 1024];

    loop {
        println!("Waiting for message...");
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let msg = String::from_utf8_lossy(&buf[..n]);
        if msg.starts_with("START") {
            eprintln!("Experiment started!");
        }
        if msg.starts_with("=") {
            eprintln!("You guessed correctly!");
            break;
        }
        if msg.starts_with(">") {
            eprintln!("Your guess is too high!");
        }
        if msg.starts_with("<") {
            eprintln!("Your guess is too low!");
        }
        if msg.starts_with("END") {
            eprintln!("Experiment ended!");
            break;
        }
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)?;
        stream.write_all(format!("GUESS {}\n", guess).as_bytes())?;
    }

    Ok(())
}