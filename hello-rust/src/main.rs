#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    address: Address,
    phone_numbers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[get("/")]
fn json() -> status::Custom<content::Json<&'static str>> {
    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let person: Person = serde_json::from_str(the_file).expect("JSON was not well-formatted");
    println!("{:?}", person);

    status::Custom(Status::ImATeapot, content::Json(""))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/config", routes![json])
}