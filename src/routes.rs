use crate::db::MainDatabase;
use crate::models::Recipe;
use mongodb::{
    bson::doc,
    results::{InsertOneResult, UpdateResult},
    Cursor,
};
use rocket::{futures::TryStreamExt, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::{Map, Value};

#[get("/")]
pub fn index() -> Json<Recipe> {
    let item = Recipe {
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

#[get("/recipes", format = "json")]
pub async fn get_recipes(db: Connection<MainDatabase>) -> Json<Vec<Recipe>> {
    let recipes: Cursor<Recipe> = db
        .database("bread")
        .collection("recipes")
        .find(None, None)
        .await
        .expect("Failed to retrieve recipes");

    Json(recipes.try_collect().await.unwrap())
}

#[get("/recipes/<id>", format = "json")]
pub async fn get_recipe(db: Connection<MainDatabase>, id: String) -> Option<Json<Recipe>> {
    let recipe = db
        .database("bread")
        .collection("recipes")
        .find_one(doc! {"_id": id}, None)
        .await
        .expect("No recipe found for given identifier");

    if let Some(r) = recipe {
        return Some(Json(r));
    }

    return None;
}

#[put("/recipes/<id>", data = "<data>", format = "json")]
pub async fn update_recipe(
    db: Connection<MainDatabase>,
    data: Json<Map<String, Value>>,
    id: String,
) -> Json<UpdateResult> {
    let res = db
        .database("bread")
        .collection::<Recipe>("recipes")
        .update_one(
            doc! {"_id": id},
            mongodb::bson::to_document(&data.into_inner()).unwrap(),
            None,
        )
        .await
        .unwrap();

    Json(res)
}

#[post("/recipes", data = "<data>", format = "json")]
pub async fn create_recipe(
    db: Connection<MainDatabase>,
    data: Json<Recipe>,
) -> Json<InsertOneResult> {
    let res = db
        .database("bread")
        .collection::<Recipe>("recipes")
        .insert_one(data.into_inner(), None)
        .await
        .expect("Error inserting recipe!");

    Json(res)
}
