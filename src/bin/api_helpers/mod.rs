#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShortApiGatewayProxyRequest {
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
}
