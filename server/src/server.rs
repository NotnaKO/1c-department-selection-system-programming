use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{error, info};

#[derive(Debug)]
enum Message {
    StartExperiment,
    Guess { client_id: usize, guess: u32 },
}

pub struct Server {
    clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>,
}

impl Server {
    const BUFFER_SIZE: usize = 1024;

    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address");
        let clients = Arc::new(Mutex::new(HashMap::new()));

        let server = Server {
            clients: clients.clone(),
        };

        let clients_for_accept = clients.clone();

        // Create register for clients
        tokio::spawn(async move {
            let mut client_id = 0;
            info!("Started listening for clients");
            while let Ok((socket, _)) = listener.accept().await {
                let clients = clients_for_accept.clone();
                client_id += 1;
                clients
                    .lock()
                    .await
                    .insert(client_id, Arc::new(Mutex::new(socket)));
                info!("Client {} connected", client_id);
            }
        });

        server
    }

    pub async fn start_experiment(&mut self, num: u32) {
        info!("Starting experiment with number {}", num);

        info!("Starting to sending START message to all clients");
        for (client_id, client) in self.clients.clone().lock().await.iter_mut() {
            info!("Sending START message to client {}", client_id);

            let _ = client.lock().await.write_all(b"START\n").await;

            // Create listener for client messages
            let client_id_copy = client_id.clone();
            let copy_clients = self.clients.clone();
            let num_copy = num.clone();
            let client_id_copy = client_id.clone();
            tokio::spawn(async move {
                let client_id = client_id_copy;
                let mut buf = [0; Self::BUFFER_SIZE];
                let socket = copy_clients
                    .lock()
                    .await
                    .get(&client_id_copy)
                    .unwrap()
                    .clone();
                loop {
                    info!("Waiting for message from client {}", client_id);
                    let n = socket.lock().await.read(&mut buf).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    info!("Received message {} from client {}", msg, client_id);
                    if msg.starts_with("GUESS") {
                        let guess = msg[6..].trim().parse::<u32>().unwrap();
                        let number = num_copy;
                        info!("Received guess {} from client {}", guess, client_id);

                        let response = if guess == number {
                            "=".to_string()
                        } else if guess < number {
                            "<".to_string()
                        } else {
                            ">".to_string()
                        };

                        info!("Sending response {} to client {}", response, client_id);
                        if let Some(client) = copy_clients.lock().await.get_mut(&client_id) {
                            let _ = client.lock().await.write_all(response.as_bytes()).await;
                        }
                    }
                }
            });

            info!("Sent START message to client {}", client_id);
        }
    }

    pub async fn shutdown(&self) {
        let mut clients = self.clients.lock().await;
        for (_, client) in clients.iter_mut() {
            let _ = client.lock().await.write_all(b"END\n").await;
        }
    }
}
