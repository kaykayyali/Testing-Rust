#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};
use serde_json;
use std::fs;
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
#[get("/")]
fn json() -> content::Json<&'static str> {
    let path = "./src/config.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    content::Json(string_to_static_str(data))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/config", routes![json])
}