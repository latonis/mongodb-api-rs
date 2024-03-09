use crate::db::MainDatabase;
use crate::models::Recipe;
use mongodb::bson::oid::ObjectId;
use mongodb::{
    bson::doc,
    results::{InsertOneResult, UpdateResult},
    Cursor,
};
use rocket::{futures::TryStreamExt, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::{json, Map, Value};

#[get("/")]
pub fn index() -> Json<Value> {
    Json(json!({"status": "It is time to make some bread!!!"}))
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
pub async fn get_recipe(db: Connection<MainDatabase>, id: &str) -> Option<Json<Recipe>> {
    let recipe = db
        .database("bread")
        .collection("recipes")
        .find_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, None)
        .await
        .expect("No recipe found for given identifier");

    if let Some(r) = recipe {
        return Some(Json(r));
    }

    return None;
}

#[patch("/recipes/<id>", data = "<data>", format = "json")]
pub async fn update_recipe(
    db: Connection<MainDatabase>,
    data: Json<Map<String, Value>>,
    id: &str,
) -> Option<Json<UpdateResult>> {
    dbg!(mongodb::bson::to_document(&data.clone().into_inner()).unwrap());

    let res = match db
        .database("bread")
        .collection::<Recipe>("recipes")
        .update_one(
            doc! {"_id": ObjectId::parse_str(id).unwrap()},
            doc! {"$set": mongodb::bson::to_document(&data.into_inner()).unwrap()},
            None,
        )
        .await {
            Ok(res) => Some(Json(res)),
            _ => None
        };
    
    res
}

#[delete("/recipes/<id>")]
pub async fn delete_recipe(db: Connection<MainDatabase>, id: &str) -> Json<Value> {
    if db
        .database("bread")
        .collection::<Recipe>("recipes")
        .delete_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, None)
        .await
        .is_err()
    {
        return Json(json!({ "status": "Recipe could not be deleted" }));
    };

    Json(json!({ "status": "Recipe successfully deleted" }))
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
