#[macro_use] extern crate rocket;
use rocket::response::{content};
use rocket::fs::{FileServer, relative};
use std::fs;

#[get("/")]
fn json() -> content::Json<&'static str> {
    let path = "./src/config.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    content::Json(Box::leak(data.into_boxed_str()))
}

#[launch]
fn rocket() -> _ {
    let takeoff = rocket::build();

    takeoff.mount("/config", routes![json])
        .mount("/", FileServer::from(relative!("static")))
}