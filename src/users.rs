use chrono::NaiveDate;

use crate::*;


#[derive(Serialize, Deserialize)]
pub struct CheckEmailResponse {
    exists: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CheckEmailJson {
    email: String
}

#[get("/check_email", data = "<email>")]
pub async fn check_email(email: Json<CheckEmailJson>, mut db: Connection<Db>) -> Json<CheckEmailResponse> {
    let result = sqlx::query!("SELECT user_id FROM users where email = ?", email.email)
        .fetch_all(&mut *db)
        .await
        .ok();
    if result.unwrap().len() > 1 {
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

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    email: String,
    password: String,
    date_of_birth: NaiveDate,
}

#[post("/signup", data = "<user_data>")]
pub async fn signup(user_data: Json<NewUser>, mut db: Connection<Db>) -> Json<SignupResponse> {
    // Check if email is in use
    let result = sqlx::query!("SELECT user_id FROM users where email = ?", user_data.email)
        .fetch_all(&mut *db)
        .await
        .ok();
    if result.unwrap().len() > 1 {
        return Json(SignupResponse {
            success: false,
            message: "Email already in use".to_string(),
        });
    };

    sqlx::query!(
        "INSERT INTO users (email, password, date_of_birth)
        VALUES (?, ?, ?)",
        user_data.email,
        user_data.password,
        user_data.date_of_birth
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
pub struct UserLogin {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    success: bool,
    user_id: i32,
}

#[get("/login", data = "<login_data>")]
pub async fn login(login_data: Json<UserLogin>, mut db: Connection<Db>) -> Json<LoginResponse> {
    let result = sqlx::query!(
        "SELECT user_id FROM users where email = ? AND password = ?",
        login_data.email,
        login_data.password
    )
    .fetch_one(&mut *db)
    .await
    .ok();

    Json(match result {
        None => LoginResponse {success: false, user_id: -1},
        Some(id) => LoginResponse{success: true, user_id: id.user_id}
    })
}
