use sqlx::SqlitePool;

use crate::{
	data::{entities::LocalApp, node_data::local_apps_repo::LocalAppsRepo},
	local_apps::stack_apps::find_deployed_local_apps,
};

pub mod app_instances;
mod coop_cloud;
pub mod region_resolver;
pub mod stack_apps;

pub async fn find_local_apps(pool: &SqlitePool) -> Result<Vec<LocalApp>, sqlx::Error> {
	let mut local_apps = find_deployed_local_apps();
	let db_apps = LocalAppsRepo::init().all(pool).await?;
	local_apps.extend(db_apps);

	Ok(local_apps)
}

pub async fn find_local_app(
	pool: &SqlitePool,
	name: &str,
	instance_id: &Option<String>,
) -> Result<Option<LocalApp>, sqlx::Error> {
	// Check Docker-deployed apps first (no DB required)
	let docker_match = find_deployed_local_apps()
		.into_iter()
		.find(|app| app.name == name && &app.instance_id == instance_id);

	if docker_match.is_some() {
		return Ok(docker_match);
	}

	// Fall back to the database
	let db_match = LocalAppsRepo::init()
		.find(pool, name, instance_id)
		.await?;

	Ok(db_match)
}
