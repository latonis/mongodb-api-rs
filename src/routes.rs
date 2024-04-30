use crate::db::MainDatabase;
use crate::models::Recipe;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, results::InsertOneResult};
use rocket::{futures::TryStreamExt, http::Status, response::status, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::{json, Map, Value};

#[get("/")]
pub fn index() -> Json<Value> {
    Json(json!({"status": "It is time to make some bread!!!"}))
}

#[get("/recipes", format = "json")]
pub async fn get_recipes(db: Connection<MainDatabase>) -> Json<Vec<Recipe>> {
    let recipes = db
        .database("bread")
        .collection("recipes")
        .find(None, None)
        .await;

    if let Ok(r) = recipes {
        if let Ok(collected) = r.try_collect::<Vec<Recipe>>().await {
            return Json(collected);
        }
    }

    return Json(vec![]);
}

#[get("/recipes/<id>", format = "json")]
pub async fn get_recipe(db: Connection<MainDatabase>, id: &str) -> Option<Json<Recipe>> {
    let b_id = ObjectId::parse_str(id);

    if b_id.is_err() {
        return None;
    }

    if let Ok(Some(recipe)) = db
        .database("bread")
        .collection("recipes")
        .find_one(doc! {"_id": b_id.unwrap()}, None)
        .await
    {
        return Some(Json(recipe));
    }

    None
}

#[put("/recipes/<id>", data = "<data>", format = "json")]
pub async fn update_recipe(
    db: Connection<MainDatabase>,
    data: Json<Map<String, Value>>,
    id: &str,
) -> Option<Json<Value>> {
    let b_id = ObjectId::parse_str(id);

    if b_id.is_err() {
        return None;
    }

    let res = match db
        .database("bread")
        .collection::<Recipe>("recipes")
        .update_one(
            doc! {"_id": b_id.as_ref().unwrap()},
            doc! {"$set": mongodb::bson::to_document(&data.into_inner()).unwrap()},
            None,
        )
        .await
    {
        Ok(_) => Some(Json(
            json!({"status": "success", "message": format!("Recipe ({}) updated successfully", b_id.unwrap())}),
        )),
        _ => None,
    };

    res
}

#[delete("/recipes/<id>")]
pub async fn delete_recipe(db: Connection<MainDatabase>, id: &str) -> status::Custom<Json<Value>> {
    let b_id = ObjectId::parse_str(id);

    if b_id.is_err() {
        return status::Custom(
            Status::NotFound,
            Json(json!({"message":"Recipe not found"})),
        );
    }

    if db
        .database("bread")
        .collection::<Recipe>("recipes")
        .delete_one(doc! {"_id": b_id.as_ref().unwrap()}, None)
        .await
        .is_err()
    {
        return status::Custom(
            Status::BadRequest,
            Json(json!({"message":format!("Recipe ({}) could not be deleted", b_id.unwrap())})),
        );
    };

    status::Custom(
        Status::Accepted,
        Json(json!({"message": format!("Recipe ({}) successfully deleted", b_id.unwrap())})),
    )
}

#[post("/recipes", data = "<data>", format = "json")]
pub async fn create_recipe(
    db: Connection<MainDatabase>,
    data: Json<Recipe>,
) -> Option<Json<InsertOneResult>> {
    if let Ok(res) = db
        .database("bread")
        .collection::<Recipe>("recipes")
        .insert_one(data.into_inner(), None)
        .await
    {
        return Some(Json(res));
    }

    None
}
