use sqlx::SqlitePool;

use crate::panda_comms::lores_events::LoResEventHeader;

pub fn header_has_region(header: &LoResEventHeader) -> Result<(), ()> {
    if header.region_id.is_some() {
        Ok(())
    } else {
        Err(())
    }
}

pub async fn region_already_projected(
    header: &LoResEventHeader,
    pool: &SqlitePool,
) -> Result<(), ()> {
    use crate::data::projections_read::regions::RegionsReadRepo;

    let region_id = match &header.region_id {
        Some(id) => id,
        None => return Err(()),
    };

    let repo = RegionsReadRepo::init();
    match repo.find(pool, &region_id.to_hex()).await {
        Ok(Some(_)) => Ok(()), // Region already projected
        Ok(None) => {
            println!("Region not projected yet.");
            Err(())
        }
        Err(e) => {
            eprintln!("Database error while checking region projection: {}", e);
            Err(())
        }
    }
}
