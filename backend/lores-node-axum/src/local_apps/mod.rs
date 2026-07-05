use sqlx::SqlitePool;

use crate::{
	data::{entities::LocalApp, node_data::local_apps_repo::LocalAppsRepo},
	local_apps::stack_apps::find_deployed_local_apps,
};

pub mod app_instances;
mod coop_cloud;
pub mod local_app_installations;
pub mod stack_apps;

pub async fn find_local_apps(pool: &SqlitePool) -> Result<Vec<LocalApp>, sqlx::Error> {
	let mut local_apps = find_deployed_local_apps();
	let db_apps = LocalAppsRepo::init().all(pool).await?;
	local_apps.extend(db_apps);

	Ok(local_apps)
}
