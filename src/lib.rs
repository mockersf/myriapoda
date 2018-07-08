extern crate http;

pub fn hello<'a>() -> http::response::Response<&'a str> {
    http::response::Builder::new()
        .status(200)
        .body("hello")
        .expect("could not build response")
}
