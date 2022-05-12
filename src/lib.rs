use diesel::Connection;
use rocket::figment::{
    map,
    value::{Map, Value},
    Figment,
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

mod controller;
mod db;
mod model;
mod schema;

fn config(db_url: &str) -> Figment {
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 20.into(),
    };
    Figment::from(rocket::Config::debug_default())
        .merge(("address", "0.0.0.0"))
        .merge(("databases", map! { "posts" => db }))
}

fn migrate(db_url: &str) {
    let mut counter = 0;
    let one_sec = std::time::Duration::from_secs(1);
    let conn = loop {
        if counter > 10 {
            panic!("Could not get database :(");
        }
        match diesel::PgConnection::establish(&db_url) {
            Ok(conn) => break conn,
            Err(_) => std::thread::sleep(one_sec),
        }
        counter += 1;
    };
    embedded_migrations::run(&conn).unwrap();
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    migrate(&db_url);

    rocket::build()
        .mount("/posts", controller::routes())
        .attach(db::Db::fairing())
        .configure(config(&db_url))
}
