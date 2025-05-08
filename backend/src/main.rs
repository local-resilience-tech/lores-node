use events::event_handler::EventHandler;
use infra::db::{run_migrations, MainDb};
use infra::spa_server::SpaServer;
use panda_comms::container::P2PandaContainer;
use panda_comms::fairing::P2PandaCommsFairing;
use panda_comms::site_events::SiteEvent;
use rocket::fairing::AdHoc;
use rocket::fs::{FileServer, Options};
use rocket::response::Redirect;
use rocket::serde::Deserialize;
use rocket::tokio;
use std::env;
use tokio::sync::mpsc;

mod events;
mod infra;
mod panda_comms;
mod repos;
mod routes;

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket_db_pools::Database;

#[get("/")]
fn hello() -> String {
    "OK".to_string()
}

#[get("/")]
fn admin_redirect() -> Redirect {
    Redirect::to("/admin")
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    frontend_asset_path: String,
}

#[launch]
#[rocket::main]
async fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("databases.main_db.url", env::var("DATABASE_URL").expect("DATABASE_URL must be set")));

    let mut rocket = rocket::custom(figment);

    let config: Config = rocket.figment().extract().expect("config");

    // log the config
    println!("Config static_asset_path: {:?}", config.frontend_asset_path);

    // state
    let (channel_tx, channel_rx): (mpsc::Sender<SiteEvent>, mpsc::Receiver<SiteEvent>) = mpsc::channel(32);
    rocket = rocket.manage(EventHandler::new(channel_rx));
    rocket = rocket.manage(P2PandaContainer::new(channel_tx));

    // fairings
    rocket = rocket
        .attach(infra::cors::cors_fairing())
        .attach(MainDb::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .attach(P2PandaCommsFairing::default());

    // frontend
    if !config.frontend_asset_path.is_empty() {
        rocket = rocket
            .mount("/admin/assets", FileServer::from(config.frontend_asset_path.clone() + "/assets").rank(3))
            .mount(
                "/admin",
                SpaServer::new(config.frontend_asset_path.clone() + "/index.html", Options::IndexFile),
            )
    }

    // routes
    rocket
        .mount("/", routes![admin_redirect])
        .mount("/hello", routes![hello])
        .mount("/api/this_site", routes::this_site::routes())
        .mount("/api/this_region", routes::this_region::routes())
        .mount("/api/this_node", routes::this_node::routes())
        .mount("/api/apps", routes::apps::routes())
}
