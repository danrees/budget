#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Deserialize,Serialize};
use rocket::{get, post, routes, FromForm, Data};
use rocket::http::{ContentType,Status};
use rocket::data::{FromData, Transformed, Outcome, Transform};
use rocket::response::{
    Stream,
    status::Custom,
};
use rocket::request::Form;
use rocket_contrib::json::*;
use rocket::config::{Config,Environment,};
use std::io::{self,Cursor,Write};
use multipart::{
    mock::StdoutTee,
    server::{
        save::Entries,
        Multipart
    },
};
use multipart::server::save::SaveResult::*;

#[post("/upload", data="<data>")]
fn multipart_uploadfile(ct: &ContentType, data: Data) -> Result<Stream<Cursor<Vec<u8>>>,Custom<String>> {
    if !ct.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-type not multipart/form-data".into()
        ));
    }
    let (_,boundry) = ct.params().find(|&(k,_)| k == "boundary").ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        )
    )?;
    match process_upload(boundry, data) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    match Multipart::with_body(data.open(),boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out)?,
        Partial(partial,reason) => {
            writeln!(out,"Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }
            process_entries(partial.entries, &mut out)?
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

fn process_entries(entries: Entries, mut out: &mut Vec<u8>) -> io::Result<()> {
    
    {
        let stdout = io::stdout();
        let tree = StdoutTee::new(&mut out, &stdout);
        entries.write_debug(tree)?;
    }
    writeln!(out, "Entries processed")
}

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

    rocket::ignite()
        .mount("/", routes![index,save_name,multipart_uploadfile])
        .launch();
}