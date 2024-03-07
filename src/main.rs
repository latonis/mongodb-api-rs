#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::MainDatabase::init()).mount(
        "/",
        routes![
            routes::index,
            routes::get_recipes,
            routes::create_recipe,
            routes::get_recipe,
            routes::delete_recipe,
        ],
    )
}
