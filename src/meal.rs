use rocket::form::Form;

use crate::*;

// Struct for a meal entry
#[derive(Debug, Serialize, Deserialize)]
pub struct Meal {
    meal_id: i32,
    meal_name: String,
    date_eaten: NaiveDateTime,
    calories: i32,
}

// Fetches all the meals in the database for a given user
#[get("/meal?<id>")]
pub async fn get_meals(id: i32, mut db: Connection<Db>) -> Option<Json<Vec<Meal>>> {
    let result =
        // Construct the SQL query, indicate the date returned should be in Meal structs
        sqlx::query_as!(Meal, "SELECT meal_id, meal_name, date_eaten, calories FROM meals WHERE user_id = ?", id)
        // execute the query
        .fetch_all(&mut *db)
        // fetch_all is async so wait to be complete
        .await
        // Convert result into option
        .ok()?;

    // Return the meals in json format
    Some(Json(result))
}

// Struct for recieving a new meal
// Does not need meal_id item as id auto_incremented
#[derive(FromForm)]
pub struct NewMeal {
    user_id: i32,
    meal_name: String,
    date_eaten: String,
    calories: i32,
}

// Inserts a new meal into the database
#[post("/meal", data = "<meal_data>")]
pub async fn new_meal(meal_data: Form<NewMeal>, mut db: Connection<Db>) {
    // Construct the query
    let result = sqlx::query!(
        "INSERT INTO meals (user_id, meal_name, date_eaten, calories)
         VALUES (?, ?, ?, ?)",
        meal_data.user_id,
        meal_data.meal_name,
        meal_data.date_eaten,
        meal_data.calories
    )
    // execute the query
    .execute(&mut *db)
    // fetch_all is async so wait to be complete
    .await;
    // Convert result into option
    
    println!("{:?}", result);

    // TODO: return status of the insert
}
