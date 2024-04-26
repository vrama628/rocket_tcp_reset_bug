#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

#[post("/", data = "<body>")]
fn index(body: Json<Vec<usize>>) -> String {
    body.len().to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
