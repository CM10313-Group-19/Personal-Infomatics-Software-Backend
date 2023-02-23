#[macro_use]
extern crate rocket;

use chrono::NaiveDateTime;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};

// Define the databse
#[derive(Database)]
#[database("test")]
struct Meals(sqlx::MySqlPool);

#[derive(Serialize, Deserialize)]
struct NewMeal {
    meal_name: String,
    date_eaten: NaiveDateTime,
    calories: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meal {
    meal_id: i32,
    meal_name: String,
    date_eaten: NaiveDateTime,
    calories: i32,
}

#[get("/meal")]
async fn get_meals(mut db: Connection<Meals>) -> Option<Json<Vec<Meal>>> {
    let r = sqlx::query_as!(Meal, "SELECT * FROM meals")
        .fetch_all(&mut *db)
        .await
        .ok()?;
    Some(Json(r))
}

#[post("/meal", data = "<meal_data>")]
async fn new_meal(meal_data: Json<NewMeal>, mut db: Connection<Meals>) {
    sqlx::query!(
        "INSERT INTO meals (meal_name, date_eaten, calories)
         VALUES (?, ?, ?)",
        meal_data.meal_name,
        meal_data.date_eaten,
        meal_data.calories
    )
    .execute(&mut *db)
    .await
    .ok();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Meals::init())
        .mount("/", routes![get_meals, new_meal])
}
