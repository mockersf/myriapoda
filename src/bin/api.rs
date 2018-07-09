extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;

extern crate http;

#[macro_use]
extern crate failure;

extern crate aws_lambda;

extern crate myriapoda;

use std::collections::HashMap;

mod api_helpers;

fn main() {
    aws_lambda::start(|req: api_helpers::ShortApiGatewayProxyRequest| {
        let response = match (req.http_method.as_ref(), req.path.as_ref()) {
            ("GET", "/welcome") => myriapoda::hello(&(&req).into()),
            ("POST", "/mirror") => myriapoda::mirror(&(&req).into()),
            (method, path) => Err(api_helpers::HttpError::UnexpectedPath {
                method: method.to_string(),
                path: path.to_string(),
            }.into()),
        };

        Ok(match response {
            Ok(response) => {
                let a: api_helpers::Response = response.into();
                a.0
            }
            Err(err) => aws_lambda::event::apigw::ApiGatewayProxyResponse {
                body: format!("{}", err),
                status_code: 500,
                is_base64_encoded: None,
                headers: HashMap::new(),
            },
        })
    })
}
