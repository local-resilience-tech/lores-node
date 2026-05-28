mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use clap::{Parser, Subcommand};
use proto::{panda_client::PandaClient, ListRegionsRequest, PublishRequest, SubscribeRequest};
use tokio::io::AsyncBufReadExt as _;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MessagePayload {
    message: String,
}

#[derive(Parser)]
#[command(name = "lores-panda", about = "CLI for the lores p2panda gRPC server")]
struct Cli {
    /// gRPC server address
    #[arg(long, default_value = "http://localhost:8201")]
    server: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Publish a message to a region
    Publish {
        /// Region ID as 64 hex characters (32 bytes)
        region: String,
        /// Message to publish (comma-delimited string)
        message: String,
    },

    /// List the regions and app namespaces the node is participating in
    ListRegions,

    /// Enter interactive live mode: publish messages line by line and print incoming messages.
    /// Press Ctrl+C to exit.
    Live {
        /// Region ID as 64 hex characters (32 bytes)
        region: String,
    },
}

const APP_NAMESPACE: &str = "lores-example-app-cli:v1";

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let server = cli.server.clone();
    match cli.command {
        Command::Publish { region, message } => {
            let region_bytes = parse_region_hex(&region)?;

            let payload_struct = MessagePayload { message };
            let mut payload_bytes: Vec<u8> = Vec::new();
            ciborium::into_writer(&payload_struct, &mut payload_bytes)
                .map_err(|e| format!("failed to encode payload as CBOR: {e}"))?;

            let mut client = connect(&server).await?;

            client
                .publish(PublishRequest {
                    region_id: region_bytes.to_vec(),
                    app_namespace: APP_NAMESPACE.to_string(),
                    payload: payload_bytes,
                })
                .await?;

            println!("published");
        }
        Command::ListRegions => {
            let mut client = connect(&server).await?;

            let response = client.list_regions(ListRegionsRequest {}).await?;

            let ids = response.into_inner().region_ids;
            if ids.is_empty() {
                println!("no registered regions");
            } else {
                for id in ids {
                    println!("{}", hex::encode(&id));
                }
            }
        }
        Command::Live { region } => {
            let region_bytes = parse_region_hex(&region)?;

            // Two separate connections: one for subscribe, one for publish.
            let mut subscribe_client = connect(&server).await?;
            let mut publish_client = connect(&server).await?;

            let stream_response = subscribe_client
                .subscribe(SubscribeRequest {
                    region_id: region_bytes.to_vec(),
                    app_namespace: APP_NAMESPACE.to_string(),
                })
                .await?;
            let mut stream = stream_response.into_inner();

            // Spawn a task that prints every incoming operation.
            tokio::spawn(async move {
                loop {
                    match stream.message().await {
                        Ok(Some(event)) => {
                            let author = hex::encode(&event.author);
                            match ciborium::from_reader::<MessagePayload, _>(
                                event.payload.as_slice(),
                            ) {
                                Ok(p) => println!("[{}...] {}", &author[..8], p.message),
                                Err(_) => {
                                    println!("[{}...] <unparseable payload>", &author[..8])
                                }
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            eprintln!("subscription stream error: {e}");
                            break;
                        }
                    }
                }
            });

            println!(
                "Live mode on region {}...\nType a message and press Enter to publish. Press Ctrl+C to exit.\n",
                &region[..8]
            );

            let stdin = tokio::io::BufReader::new(tokio::io::stdin());
            let mut lines = stdin.lines();

            loop {
                tokio::select! {
                    line = lines.next_line() => {
                        match line? {
                            Some(text) if !text.trim().is_empty() => {
                                let payload_struct = MessagePayload { message: text };
                                let mut payload_bytes: Vec<u8> = Vec::new();
                                ciborium::into_writer(&payload_struct, &mut payload_bytes)
                                    .map_err(|e| format!("failed to encode payload as CBOR: {e}"))?;

                                publish_client
                                    .publish(PublishRequest {
                                        region_id: region_bytes.to_vec(),
                                        app_namespace: APP_NAMESPACE.to_string(),
                                        payload: payload_bytes,
                                    })
                                    .await?;
                            }
                            Some(_) => {} // blank line, ignore
                            None => break, // stdin closed (EOF)
                        }
                    }
                    _ = tokio::signal::ctrl_c() => {
                        println!("\nexiting live mode");
                        std::process::exit(0);
                    }
                }
            }
        }
    }

    Ok(())
}

async fn connect(
    server: &str,
) -> Result<PandaClient<tonic::transport::Channel>, Box<dyn std::error::Error>> {
    PandaClient::connect(server.to_string())
        .await
        .map_err(|e| format!("could not connect to gRPC server at {server}: {e}").into())
}

fn parse_region_hex(s: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let bytes = hex::decode(s).map_err(|e| format!("invalid region hex: {e}"))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "region must be exactly 32 bytes (64 hex characters)")?;
    Ok(arr)
}
