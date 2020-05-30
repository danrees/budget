#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Deserialize,Serialize};
use rocket::{get,post,routes,FromForm};
use rocket::request::Form;
use rocket_contrib::json::*;

#[derive(Debug,FromForm,Deserialize,Serialize)]
struct Person {
    name: String,
}

#[get("/")]
fn index() -> Json<Person> {
    Json(Person{name: String::from("world")})
}

#[post("/", data = "<input>")]
fn save_name(input: Json<Person>) -> String {
    format!("Hello {}",input.name)
}

fn main() {
    //Need to set up a webserver

    rocket::ignite().mount("/", routes![index,save_name]).launch();
}