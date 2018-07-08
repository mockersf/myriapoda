extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate http;

extern crate aws_lambda;

extern crate myriapoda;

mod api_helpers;

fn main() {
    aws_lambda::start(|req: api_helpers::ShortApiGatewayProxyRequest| {
        let response = match (req.http_method.as_ref(), req.path.as_ref()) {
            ("GET", "/welcome") => myriapoda::hello(&(&req).into()),
            ("POST", "/mirror") => myriapoda::mirror(&(&req).into()),
            (_, _) => unreachable!(),
        };

        let a: api_helpers::Response = response.into();
        Ok(a.0)
    })
}
