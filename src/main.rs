#[macro_use]
extern crate rocket;


// Rocket application launches from here
#[launch] 
fn rocket() -> _ {
    rocket::build().mount("/", routes![])
}
