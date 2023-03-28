#[macro_use]
extern crate rocket;

pub use chrono::NaiveDateTime;
pub use rocket::serde::json::Json;
pub use rocket_db_pools::{Connection, Database};
pub use serde::{Deserialize, Serialize};

mod meal;
mod users;
mod weight;

use crate::meal::{get_meals, new_meal};
use crate::users::{check_email, login, signup};
use crate::weight::{get_weight, get_weights, new_weight};

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
        .mount(
            "/",
            routes![
                get_weight,
                get_weights,
                new_weight,
                get_meals,
                new_meal,
                signup,
                check_email,
                login
            ],
        )
}
