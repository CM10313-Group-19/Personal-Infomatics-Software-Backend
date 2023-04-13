use chrono::{NaiveDate, NaiveTime};
use rocket::form::Form;

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Sleep {
    sleep_id: i32,
    start_time: NaiveTime,
    end_time: NaiveTime,
    sleep_date: NaiveDate,
}

#[derive(FromForm)]
pub struct NewSleep {
    user_id: i32,
    start_time: String,
    end_time: String,
    sleep_date: String,
}

/// Get all recorded sleeps for a user
#[get("/sleep?<id>")]
pub async fn get_sleep(id: i32, mut db: Connection<Db>) -> Json<Option<Vec<Sleep>>> {
    let result = sqlx::query_as!(
        Sleep,
        "SELECT sleep_id, start_time, end_time, sleep_date 
        FROM sleep WHERE user_id = ?
        ORDER BY sleep_date",
        id
    )
    .fetch_all(&mut *db)
    .await
    .ok();
    Json(result)
}

/// Add a new sleep record
#[post("/sleep", data = "<new_sleep>")]
pub async fn new_sleep(new_sleep: Form<NewSleep>, mut db: Connection<Db>) {
    sqlx::query!(
        "INSERT INTO sleep (user_id, start_time, end_time, sleep_date)
        VALUES (?, ?, ?, ?)",
        new_sleep.user_id,
        new_sleep.start_time,
        new_sleep.end_time,
        new_sleep.sleep_date
    )
    .execute(&mut *db)
    .await
    .ok();
}
