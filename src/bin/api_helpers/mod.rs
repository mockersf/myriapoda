use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use aws_lambda;
use http;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShortApiGatewayProxyRequest {
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[serde(default, deserialize_with = "nullable_default")]
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

fn nullable_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(T::default))
}

pub struct Response(pub aws_lambda::event::apigw::ApiGatewayProxyResponse);

impl<'a> From<http::response::Response<&'a str>> for Response {
    fn from(response: http::response::Response<&'a str>) -> Self {
        Response(aws_lambda::event::apigw::ApiGatewayProxyResponse {
            body: response.body().to_string(),
            status_code: response.status().as_u16() as i64,
            is_base64_encoded: None,
            headers: HashMap::new(),
        })
    }
}

impl<'a> Into<http::request::Request<&'a str>> for &'a ShortApiGatewayProxyRequest {
    fn into(self) -> http::request::Request<&'a str> {
        let method: &str = self.http_method.as_ref();
        let path: &str = self.path.as_ref();

        let mut builder = http::request::Request::builder();
        builder.method(method).uri(path);
        self.headers.iter().for_each(|(key, value)| {
            let key: &str = key.as_ref();
            let value: &str = value.as_ref();
            builder.header(key, value);
        });

        if let Some(ref body) = self.body {
            builder.body(body.as_ref())
        } else {
            builder.body("")
        }.expect("Couldn't build request")
    }
}
