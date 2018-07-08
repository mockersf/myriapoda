extern crate http;

pub fn hello<'a>(_req: &http::request::Request<&str>) -> http::response::Response<&'a str> {
    http::response::Builder::new()
        .status(200)
        .body("hello")
        .expect("could not build response")
}

pub fn mirror<'a>(req: &http::request::Request<&'a str>) -> http::response::Response<&'a str> {
    http::response::Builder::new()
        .status(200)
        .body(*req.body())
        .expect("could not build response")
}
