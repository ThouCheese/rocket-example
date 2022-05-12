use rocket_sync_db_pools::database;

#[database("posts")]
pub struct Db(diesel::PgConnection);
