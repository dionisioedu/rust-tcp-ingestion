use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use mongodb::{Client, options::ClientOptions, Database};
use serde::Serialize;
use serde::Deserialize;
use log::{info, error};

#[derive(Debug, Serialize, Deserialize)]
struct IngestMessage {
    source: String,
    value: f64,
    timestamp: String,
}

async fn handle_client(mut socket: tokio::net::TcpStream, db: Database) {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                // Conexão fechada
                break;
            }
            Ok(n) => {
                let data = &buffer[..n];
                let message = String::from_utf8_lossy(data);

                info!("Received: {}", message);

                // Tenta fazer o parsing do JSON
                match serde_json::from_str::<IngestMessage>(&message) {
                    Ok(parsed) => {
                        let collection = db.collection::<IngestMessage>("logs");
                        if let Err(e) = collection.insert_one(parsed, None).await {
                            error!("Failed to insert into MongoDB: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Invalid JSON received: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://mongo:27017".to_string());
    let client_options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("ingestion");

    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    info!("Listening on 0.0.0.0:7878");

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            handle_client(socket, db).await;
        });
    }
}
