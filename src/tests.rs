use super::rocket;

use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_get_file() {
    let client = Client::new(rocket()).unwrap();
    let response = client.get("/test/test_image.jpg").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_load_image() {
    let client = Client::new(rocket()).unwrap();
    let query = "/load?url=https://cdn.stocksnap.io/img-thumbs/960w/ERCAGPLWDF.jpg".to_string();
    let mut response = client.get(query).dispatch();
    let imgtag = "<img src=\"/filtered_ERCAGPLWDF.jpg\" alt=\"filtered_ERCAGPLWDF.jpg\" width=\"100\" height=\"100\">";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(format!("{}", imgtag)))
}

#[test]
fn test_load_two_images() {
    let client = Client::new(rocket()).unwrap();
    let query = "/load?url=https://cdn.stocksnap.io/img-thumbs/960w/6VOP7FYDYI.jpg;https://cdn.stocksnap.io/img-thumbs/960w/VCJK4NBK4W.jpg".to_string();
    let mut response = client.get(query).dispatch();
    let imgtag1 = "<img src=\"/filtered_6VOP7FYDYI.jpg\" alt=\"filtered_6VOP7FYDYI.jpg\" width=\"100\" height=\"100\">";
    let imgtag2 = "<img src=\"/filtered_VCJK4NBK4W.jpg\" alt=\"filtered_VCJK4NBK4W.jpg\" width=\"100\" height=\"100\">";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(format!("{}{}", imgtag1, imgtag2)))
}