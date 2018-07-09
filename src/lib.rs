extern crate failure;
extern crate http;

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
