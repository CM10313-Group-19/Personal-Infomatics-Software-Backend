use chrono::NaiveDate;
use rocket::form::Form;

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct CheckEmailResponse {
    exists: bool,
}

#[get("/check_email?<email>")]
pub async fn check_email(
    email: String,
    mut db: Connection<Db>,
) -> Json<CheckEmailResponse> {
    let result = sqlx::query!("SELECT user_id FROM users where email = ?", email)
        .fetch_all(&mut *db)
        .await
        .ok();
    if result.unwrap().len() > 0 {
        Json(CheckEmailResponse { exists: true })
    } else {
        Json(CheckEmailResponse { exists: false })
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    success: bool,
    message: String,
}

#[derive(FromForm)]
pub struct NewUser {
    email: String, 
    password: String,
    date_of_birth: String,
}

#[post("/signup", data = "<new_user>")]
pub async fn signup(new_user: Form<NewUser>, mut db: Connection<Db>) -> Json<SignupResponse> {
    // Check if email is in use
    let result = sqlx::query!("SELECT user_id FROM users where email = ?", new_user.email)
        .fetch_all(&mut *db)
        .await
        .ok();

    if result.unwrap().len() > 0 {
        return Json(SignupResponse {
            success: false,
            message: "Email already in use".to_string(),
        });
    };

    // Attempt to parse date of birth to naive date
    let date = NaiveDate::parse_from_str(&new_user.date_of_birth, "%Y-%m-%d");
    if date.is_err() {
        return Json(SignupResponse {
            success: false,
            message: "Date must be in the format: Y-m-d".to_string(),
        });
    } 

    sqlx::query!(
        "INSERT INTO users (email, password, date_of_birth)
        VALUES (?, ?, ?)",
        new_user.email,
        new_user.password,
        date.unwrap()
    )
    .execute(&mut *db)
    .await
    .ok();

    Json(SignupResponse {
        success: true,
        message: "".to_string(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    success: bool,
    user_id: i32,
}

#[get("/login?<email>&<password>")]
pub async fn login(email: String, password: String, mut db: Connection<Db>) -> Json<LoginResponse> {
    let result = sqlx::query!(
        "SELECT user_id FROM users where email = ? AND password = ?",
        email,
        password
    )
    .fetch_one(&mut *db)
    .await
    .ok();

    Json(match result {
        None => LoginResponse {
            success: false,
            user_id: -1,
        },
        Some(id) => LoginResponse {
            success: true,
            user_id: id.user_id,
        },
    })
}
