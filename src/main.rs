#[macro_use]
extern crate rocket;

mod db;
mod models;

use db::MainDatabase;
use models::Recipe;
use mongodb::{results::InsertOneResult, Cursor};
use rocket::{futures::TryStreamExt, serde::json::Json};
use rocket_db_pools::{Connection, Database};

#[get("/")]
fn index() -> Json<models::Recipe> {
    let item = models::Recipe {
        id: None,
        title: "Sourdough".to_string(),
        ingredients: vec![
            "yeast".to_string(),
            "water".to_string(),
            "flour".to_string(),
            "salt".to_string(),
        ],
        temperature: 475,
        bake_time: 45,
    };
    Json(item)
}

#[get("/recipes")]
async fn get_bread(db: Connection<MainDatabase>) -> Json<Vec<models::Recipe>> {
    let recipes: Cursor<Recipe> = db
        .database("bread")
        .collection("recipes")
        .find(None, None)
        .await
        .expect("Failed to retrieve recipes");

    Json(recipes.try_collect().await.unwrap())
}

#[post("/recipes", data = "<data>")]
async fn post_recipe(db: Connection<MainDatabase>, data: Json<Recipe>) -> Json<InsertOneResult> {
    let res = db
        .database("bread")
        .collection::<Recipe>("recipes")
        .insert_one(data.into_inner(), None)
        .await
        .expect("Error inserting recipe!");

    Json(res)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::MainDatabase::init())
        .mount("/", routes![index, get_bread, post_recipe])
}
