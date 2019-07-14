#![feature(proc_macro_hygiene, decl_macro)]
#[cfg(test)] mod tests;

#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate image;

use rocket::response::NamedFile;
use rocket::Response;
use rocket::http::Status;
use rocket::http::ContentType;
use std::io;
use std::io::Cursor;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::result::Result;
use image::DynamicImage;
use image::ImageResult;

fn filter(img: DynamicImage) -> ImageResult<DynamicImage> {
    let kernel = [-1.0f32, -1.0, -1.0,
                  -1.0, 8.0, -1.0,
                  -1.0, -1.0, -1.0];
    Ok(img.filter3x3(&kernel))
}

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
            Err(_e) => {}
            Ok(mut resp) => {
                let filename = &format!("filtered_{}", link.rsplit('/').next().unwrap());
                let path = &format!("/{}", filename);
                let localpath = &format!("static/images/{}", filename);
                let mut file = File::create(localpath).expect("Failed to create file");
                io::copy(&mut resp, &mut file).expect("Failed to copy content");
                let mut img = image::open(localpath).expect("Failed to open image");
                img = filter(img).expect("Failed to filter image");
                img.save(localpath).expect("Failed to save image");
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

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![load, files])
}

fn main() {
    rocket().launch();
}