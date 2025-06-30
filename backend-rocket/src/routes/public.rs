use rocket::serde;
use rocket::Route;
use rocket::State;
use rocket_db_pools::Connection;
use serde::json::Json;
use serde::Serialize;

use crate::infra::db::MainDb;
use crate::panda_comms::container::P2PandaContainer;
use crate::repos::this_node::ThisNodeRepo;
use crate::repos::this_node::ThisNodeRepoError;

#[derive(Serialize)]
enum HelloStatus {
    Up,
    Down,
}

#[derive(Serialize)]
struct HelloResponse {
    web: HelloStatus,
    db: HelloStatus,
    p2p: HelloStatus,
    node: Option<HelloResponseNode>,
}

#[derive(Serialize)]
struct HelloResponseNode {
    id: String,
    name: String,
}

#[get("/hello")]
async fn hello(mut db: Connection<MainDb>, panda_container: &State<P2PandaContainer>) -> Json<HelloResponse> {
    let mut response = HelloResponse {
        web: HelloStatus::Up,
        db: HelloStatus::Down,
        p2p: HelloStatus::Down,
        node: None,
    };

    match panda_container.get_public_key().await {
        Ok(_) => {
            response.p2p = HelloStatus::Up;
        }
        Err(e) => {
            println!("Failed to get public key: {:?}", e);
            response.web = HelloStatus::Down;
        }
    }

    let repo = ThisNodeRepo::init();

    let node_result = repo.find(&mut db).await.map(|node| Json(node));
    match node_result {
        Ok(node) => {
            response.db = HelloStatus::Up;
            response.node = Some(HelloResponseNode {
                id: node.id.to_string(),
                name: node.name.to_string(),
            });
        }
        Err(e) => match e {
            ThisNodeRepoError::NotFound(_) => {
                response.db = HelloStatus::Up;
                println!("Node not found");
            }
            ThisNodeRepoError::InternalServerError(_) => {
                response.db = HelloStatus::Down;
            }
        },
    }

    Json(response)
}

pub fn routes() -> Vec<Route> {
    routes![hello]
}
