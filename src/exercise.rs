use chrono::{NaiveDate, NaiveTime};
use rocket::form::Form;

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Exercise {
    exercise_id: i32,
    duration: NaiveTime,
    distance: f32,
    exercise_type: String,
    exercise_date: NaiveDate,
}

#[derive(FromForm)]
pub struct NewExercise {
    user_id: i32,
    duration: String,
    distance: f32,
    exercise_type: String,
    exercise_date: String,
}

#[get("/exercise?<id>")]
pub async fn get_exercise(id: i32, mut db: Connection<Db>) -> Json<Option<Vec<Exercise>>> {
    let result = sqlx::query_as!(
        Exercise,
        "SELECT exercise_id, duration, distance, exercise_type, exercise_date
        FROM exercise WHERE user_id = ?
        ORDER BY exercise_date",
        id
    )
    .fetch_all(&mut *db)
    .await
    .ok();
    Json(result)
}

#[post("/exercise", data = "<new_exercise>")]
pub async fn new_exercise(new_exercise: Form<NewExercise>, mut db: Connection<Db>) {
    sqlx::query!(
        "INSERT INTO exercise (user_id, duration, distance, exercise_type, exercise_date)
        VALUES (?, ?, ?, ?, ?)",
        new_exercise.user_id,
        new_exercise.duration,
        new_exercise.distance,
        new_exercise.exercise_type,
        new_exercise.exercise_date
    )
    .execute(&mut *db)
    .await
    .ok();
}
