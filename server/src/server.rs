use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tracing::{error, info};

#[derive(Debug)]
enum Message {
    StartExperiment,
    Guess { client_id: usize, guess: u32 },
    Response { client_id: usize, response: String },
}

#[derive(Debug)]
enum ServerState {
    Idle,
    Experiment { number: u32 },
}

pub struct Server {
    clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
    sender: mpsc::Sender<Message>,

    state: ServerState,
}

impl Server {
    const CHANNEL_CAPACITY: usize = 100;
    const BUFFER_SIZE: usize = 1024;

    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");
        let (sender, mut receiver) = mpsc::channel(Self::CHANNEL_CAPACITY);
        let clients = Arc::new(Mutex::new(HashMap::new()));

        let server = Server {
            clients: clients.clone(),
            sender: sender.clone(),
            state: ServerState::Idle,
        };

        let clients_for_accept = clients.clone();

        tokio::spawn(async move {
            let mut client_id = 0;
            info!("Started listening for clients");
            while let Ok((socket, _)) = listener.accept().await {
                let clients = clients_for_accept.clone();
                let sender = sender.clone();
                client_id += 1;
                clients
                    .lock()
                    .await
                    .insert(client_id, Arc::new(Mutex::new(socket)));
                info!("Client {} connected", client_id);
                let client_id_copy = client_id;

                tokio::spawn(async move {
                    let mut buf = [0; Self::BUFFER_SIZE];
                    let socket = clients.lock().await.get(&client_id_copy).unwrap().clone();
                    loop {
                        let n = socket.lock().await.read(&mut buf).await.unwrap();
                        if n == 0 {
                            break;
                        }
                        let msg = String::from_utf8_lossy(&buf[..n]);
                        if msg.starts_with("GUESS") {
                            let guess = msg[6..].trim().parse().unwrap();
                            sender
                                .send(Message::Guess {
                                    client_id: client_id_copy,
                                    guess,
                                })
                                .await
                                .unwrap();
                        }
                    }
                });
            }
        });

        tokio::spawn(async move {
            info!("Started message handling");
            while let Some(message) = receiver.recv().await {
                match message {
                    Message::StartExperiment => {
                        let mut clients = clients.lock().await;
                        for (_, client) in clients.iter_mut() {
                            let _ = client.lock().await.write_all(b"START\n").await;
                        }
                    }
                    Message::Guess { client_id, guess } => {
                        // Handle guess logic here
                    }
                    Message::Response {
                        client_id,
                        response,
                    } => {
                        let mut clients = clients.lock().await;
                        if let Some(client) = clients.get_mut(&client_id) {
                            let _ = client.lock().await.write_all(response.as_bytes()).await;
                        }
                    }
                }
            }
        });

        server
    }

    pub async fn start_experiment(&self) {
        self.sender.send(Message::StartExperiment).await.unwrap();
    }

    async fn shutdown(&self) {
        let mut clients = self.clients.lock().await;
        for (_, client) in clients.iter_mut() {
            let _ = client.lock().await.write_all(b"END\n").await;
        }
    }
}
