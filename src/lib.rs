use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// Establishes a connection to the mySql database url stored in the `DATEBASE_URL` environment
/// variable
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATEBASE_URL").expect("DATEBASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
