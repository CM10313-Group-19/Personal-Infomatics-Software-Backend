#[macro_use]
extern crate rocket;

pub use chrono::NaiveDateTime;
pub use rocket::serde::json::Json;
pub use rocket_db_pools::{Connection, Database};
pub use serde::{Deserialize, Serialize};

mod meal;
mod users;

use crate::meal::{get_meals, new_meal};
use crate::users::{signup, check_email, login};

// Define the databse
#[derive(Database)]
#[database("test")]
pub struct Db(sqlx::MySqlPool);

// Startpoint of application
#[launch]
fn rocket() -> _ {
    rocket::build()
        // Attach the database connection
        .attach(Db::init())
        // Mount the routes
        .mount("/", routes![get_meals, new_meal, signup, check_email, login])
}
