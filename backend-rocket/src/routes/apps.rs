use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Route;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppList {
    apps: Vec<String>,
}

// A type which either has an AppList or an error
type AppListResult = Result<AppList, String>;

#[get("/installed", format = "json")]
fn installed_apps() -> Json<AppListResult> {
    Json(Ok(AppList {
        apps: vec!["foo".to_string(), "bar".to_string()],
    }))
}

pub fn routes() -> Vec<Route> {
    routes![installed_apps]
}

// Json(Ok(AppList {
//     foo: "bar".to_string(),
// }))

// match Docker::connect() {
//     // we consider the local Docker socket by default, with the default path (/var/run/docker.sock), no need to precise the path
//     Ok(mut docker) => match docker.get_images(false) {
//         Ok(containers) => Json(Ok(AppList {
//             apps: containers
//                 .iter()
//                 .map(|image| image.Id.clone())
//                 .collect::<Vec<String>>(),
//         })),
//         Err(e) => Json(Err(e.to_string())),
//     },
//     Err(_e) => Json(Err("no_docker".to_string())),
// }
