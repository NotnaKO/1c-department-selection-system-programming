mod server;
mod leaderboard;

use clap::Parser;
use server::Server;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;

#[derive(Parser)]
struct Args {
    /// Address to bind the server to
    #[clap(short, long, default_value = "127.0.0.1:8080")]
    address: String,

    /// Filename to store log files
    #[clap(short, long, default_value = "server.log")]
    log_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let file_appender = RollingFileAppender::new(Rotation::MINUTELY, "log", &args.log_file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_writer(non_blocking)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    let address = &args.address;

    info!("Starting server at {}", address);
    let mut server = Server::new(&args.address).await;
    info!("Server started");

    loop {
        let mut input = String::with_capacity(10);
        println!("Enter a number to experiment:");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read input: {}", e);
                continue;
            }
        }
        match input.trim().parse::<u32>() {
            Ok(num) => {
                server.start_experiment(num).await;
                println!(
                    "Experiment started, now start register waiters, waiting for 'exit', 'start' or 'stat'"
                );
                input.clear();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "exit" {
                    break;
                }
                if input.trim() == "stat" {
                    let scores = server.leaderboard.lock().await.get_scores().await;
                    for (client, score) in scores {
                        println!("Client {} scored {}", client, score);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse input: {}", e);
            }
        }
    }

    info!("Shutting down server");
    server.shutdown().await;
    info!("Server shut down");
    println!("\nCompleted! All logs are stored in {}", args.log_file);
}
