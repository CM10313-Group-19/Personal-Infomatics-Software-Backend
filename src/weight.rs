use crate::*;
use chrono::NaiveDate;
use rocket::form::Form;

// Structs for sending and retrieving weight data

#[derive(Serialize, Deserialize)]
pub struct Weight {
    weight_id: i32,
    day_measured: NaiveDate,
    value: i32,
}

#[derive(FromForm)]
pub struct NewWeight {
    user_id: i32,
    weight_id: i32,
    day_measured: String,
    value: i32,
}

/// Get the last recorded weight for a user
#[get("/weight?<id>")]
pub async fn get_weight(id: i32, mut db: Connection<Db>) -> Json<Option<Weight>> {
    let result = sqlx::query_as!(
        Weight,
        "SELECT weight_id, day_measured, value FROM weights WHERE user_id = ? ORDER BY day_measured",
        id
    ).fetch_one(&mut *db).await.ok();
    Json(result)
}

/// Get the all weights for a user
#[get("/weights?<id>")]
pub async fn get_weights(id: i32, mut db: Connection<Db>) -> Json<Option<Vec<Weight>>> {
    let result = sqlx::query_as!(
        Weight,
        "SELECT weight_id, day_measured, value FROM weights WHERE user_id = ? ORDER BY day_measured",
        id
    ).fetch_all(&mut *db).await.ok();
    Json(result)
}

/// Store a new weight for a user
#[post("/weight", data = "<new_weight>")]
pub async fn new_weight(new_weight: Form<NewWeight>, mut db: Connection<Db>) {
    sqlx::query!(
        "INSERT INTO weights (user_id, weight_id, day_measured, value)
        VALUES (?, ?, ?, ?)",
        new_weight.user_id,
        new_weight.weight_id,
        new_weight.day_measured,
        new_weight.value
    )
    .execute(&mut *db)
    .await
    .ok();
}
