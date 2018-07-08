extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate aws_lambda as lambda;

extern crate myriapoda;

use std::collections::HashMap;

use lambda::event::apigw::ApiGatewayProxyResponse;

mod api_helpers;

fn main() {
    lambda::start(|req: api_helpers::ShortApiGatewayProxyRequest| {
        let response = match (req.http_method.as_ref(), req.path.as_ref()) {
            ("GET", "/welcome") => myriapoda::hello(),
            (_, _) => unreachable!(),
        };

        Ok(ApiGatewayProxyResponse {
            body: response.body().to_string(),
            status_code: response.status().as_u16() as i64,
            is_base64_encoded: None,
            headers: HashMap::new(),
        })
    })
}
