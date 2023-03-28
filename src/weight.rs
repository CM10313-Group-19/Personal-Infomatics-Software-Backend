use crate::*;
use chrono::NaiveDate;

// Structs for sending and retriving weight data

#[derive(Serialize, Deserialize)]
struct WeightId {
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct Weight {
    weight_id: i32,
    day_measured: NaiveDate,
    value: i32,
}

#[derive(Serialize, Deserialize)]
struct NewWeight {
    user_id: i32,
    weight_id: i32,
    day_measured: NaiveDate,
    value: i32,
}

#[get("/weight", data = "<id>")]
async fn get_weight(id: Json<WeightId>, mut db: Connection<Db>) -> Json<Option<Weight>> {
    let result = sqlx::query_as!(
        Weight,
        "SELECT weight_id, day_measured, value FROM weights WHERE user_id = ? ORDER BY day_measured",
        id.id
    ).fetch_one(&mut *db).await.ok();
    Json(result)
}

#[get("/weights", data = "<id>")]
async fn get_weights(id: Json<WeightId>, mut db: Connection<Db>) -> Json<Option<Vec<Weight>>> {
    let result = sqlx::query_as!(
        Weight,
        "SELECT weight_id, day_measured, value FROM weights WHERE user_id = ? ORDER BY day_measured",
        id.id
    ).fetch_all(&mut *db).await.ok();
    Json(result)
}

#[post("/weight", data = "<new_weight>")]
async fn new_weight(new_weight: Json<NewWeight>, mut db: Connection<Db>) {
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
