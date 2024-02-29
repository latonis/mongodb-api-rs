use rocket_db_pools::{mongodb::Client, Database};

#[derive(Database)]
#[database("bread")]
pub struct MainDatabase(Client);
