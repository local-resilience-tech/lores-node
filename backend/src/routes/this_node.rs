use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::Route;
use rocket::{post, State};
use rocket_db_pools::Connection;

use crate::infra::db::MainDb;
use crate::panda_comms::container::P2PandaContainer;
use crate::panda_comms::lores_events::{LoResEventPayload, NodeAnnouncedData, NodeUpdatedData};
use crate::repos::entities::Node;
use crate::repos::this_node::{ThisNodeRepo, ThisNodeRepoError};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateNodeDetails {
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct UpdateNodeDetails {
    name: String,
    public_ipv4: String,
}

#[post("/create", data = "<data>")]
async fn create(data: Json<CreateNodeDetails>, panda_container: &State<P2PandaContainer>) -> Result<Json<Node>, ThisNodeRepoError> {
    let event_payload = LoResEventPayload::NodeAnnounced(NodeAnnouncedData { name: data.name.clone() });

    panda_container
        .publish_persisted(event_payload)
        .await
        .map_err(|e| {
            println!("got error: {}", e);
            ThisNodeRepoError::InternalServerError(e.to_string())
        })?;

    return Ok(Json(Node {
        id: "1".to_string(),
        name: data.name.clone(),
    }));
}

#[get("/", format = "json")]
async fn show(mut db: Connection<MainDb>) -> Result<Json<Node>, ThisNodeRepoError> {
    let repo = ThisNodeRepo::init();

    repo.find(&mut db).await.map(|node| Json(node))
}

#[patch("/", format = "json", data = "<data>")]
async fn update(data: Json<UpdateNodeDetails>, panda_container: &State<P2PandaContainer>) -> Result<Json<Node>, ThisNodeRepoError> {
    println!("update node: {:?}", data);

    let event_payload = LoResEventPayload::NodeUpdated(NodeUpdatedData {
        name: data.name.clone(),
        public_ipv4: data.public_ipv4.clone(),
    });

    panda_container
        .publish_persisted(event_payload)
        .await
        .map_err(|e| {
            println!("got error: {}", e);
            ThisNodeRepoError::InternalServerError(e.to_string())
        })?;

    return Ok(Json(Node {
        id: "1".to_string(),
        name: data.name.clone(),
    }));
}

pub fn routes() -> Vec<Route> {
    routes![create, show, update]
}
