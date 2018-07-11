extern crate chrono;
extern crate failure;
extern crate http;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod model;

pub fn hello<'a>(
    _req: &http::request::Request<&str>,
) -> Result<http::response::Response<&'a str>, failure::Error> {
    Ok(http::response::Builder::new().status(200).body("hello")?)
}

pub fn mirror<'a>(
    req: &http::request::Request<&'a str>,
) -> Result<http::response::Response<&'a str>, failure::Error> {
    Ok(http::response::Builder::new()
        .status(200)
        .body(*req.body())?)
}

pub fn new_conference<'a>(
    req: &http::request::Request<model::Conference>,
) -> Result<http::response::Response<&'a str>, failure::Error> {
    Ok(http::response::Builder::new().status(200).body("done")?)
}
