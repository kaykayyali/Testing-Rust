#[macro_use] extern crate rocket; // Rockets core crate
use rocket::response::{content}; // Load the content crate, which helps with json. This seems to not be the best way
use rocket::fs::{FileServer, relative}; // Explicit loading of FS dependency to handle general file linking
use std::fs; // Standard filesystem, used to read the json file used as the basis for the examples

#[get("/")] // Special decorator, need to learn. unique to version 0.5+
fn json() -> content::Json<&'static str> {
    let path = "./src/config.json"; // This should probably be in another dir. :shrug:
    let data = fs::read_to_string(path).expect("Unable to read file"); // Reads file to some kind of special string type, that !== to &str
    content::Json(Box::leak(data.into_boxed_str())) // Weird conversion that mutates some form of dynamic string into a static string. Probably related to buffers
}

#[launch] // Special decorator, need to learn. unique to version 0.5+, this version doesn't like you calling launch on your own for some reason

//The primary launcher for the server. Mounts routes, and serves as a place for configuration if needed.
fn rocket() -> _ {
    let takeoff = rocket::build();

    takeoff.mount("/config", routes![json]) // I need to understand wth this does.
        .mount("/", FileServer::from(relative!("static"))) // Responsible for using the FS helper, to handle supporting requests like /path/to/file.txt
}