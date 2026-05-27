mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use clap::{Parser, Subcommand};
use proto::{panda_client::PandaClient, ListTopicsRequest, PublishRequest};

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
    /// Publish a payload to a topic
    Publish {
        /// Topic ID as 64 hex characters (32 bytes)
        topic: String,
        /// Payload to publish (sent as UTF-8 bytes)
        payload: String,
    },

    /// List the topics the node is currently subscribed to
    ListTopics,
}

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
        Command::Publish { topic, payload } => {
            let topic_bytes = parse_topic_hex(&topic)?;

            let mut client = connect(&server).await?;

            client
                .publish(PublishRequest {
                    topic_id: topic_bytes.to_vec(),
                    payload: payload.into_bytes(),
                })
                .await?;

            println!("published");
        }
        Command::ListTopics => {
            let mut client = connect(&server).await?;

            let response = client.list_topics(ListTopicsRequest {}).await?;

            let topics = response.into_inner().topic_ids;
            if topics.is_empty() {
                println!("no subscribed topics");
            } else {
                for id in topics {
                    println!("{}", hex::encode(id));
                }
            }
        }
    }

    Ok(())
}

async fn connect(server: &str) -> Result<PandaClient<tonic::transport::Channel>, Box<dyn std::error::Error>> {
    PandaClient::connect(server.to_string())
        .await
        .map_err(|e| format!("could not connect to gRPC server at {server}: {e}").into())
}

fn parse_topic_hex(s: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let bytes = hex::decode(s).map_err(|e| format!("invalid topic hex: {e}"))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| "topic must be exactly 32 bytes (64 hex characters)")?;
    Ok(arr)
}
