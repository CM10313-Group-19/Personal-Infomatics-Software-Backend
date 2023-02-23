#[macro_use]
extern crate rocket;
use rocket_db_pools::Database;

mod meal;

use crate::meal::{get_meals, new_meal};

// Define the databse
#[derive(Database)]
#[database("test")]
pub struct Db(sqlx::MySqlPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![get_meals, new_meal])
}
