use crate::*;
use chrono::NaiveDate;

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
