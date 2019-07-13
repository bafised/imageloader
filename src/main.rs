#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;

use rocket::response::NamedFile;
use rocket::Response;
use rocket::http::Status;
use rocket::http::ContentType;
use std::io;
use std::io::Cursor;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/images/").join(file)).ok()
}

#[get("/load?<url>")]
fn load(url: String) -> Result<Response<'static>, Status> {
    let links = url.split(";");
    let mut content = String::new();

    for link in links {
        match reqwest::get(link) {
            Err(e) => {

            }

            Ok(mut resp) => {
                let filename = link.rsplit('/').next().unwrap();
                let path = &format!("/{}", filename);
                let localpath = &format!("static/images/{}", filename);
                let mut file = File::create(localpath).expect("Failed to create file");
                io::copy(&mut resp, &mut file).expect("Failed to copy content");
                content.push_str(&format!("<img src=\"{}\" alt=\"{}\" width=\"100\" height=\"100\">", path, filename));
            }
        };
    }

    let mut response : Response = Response::new();
    response.set_status(Status::Ok);
    response.set_header(ContentType::HTML);
    response.set_sized_body(Cursor::new(content));
    Ok(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![load, files])
        .launch();
}