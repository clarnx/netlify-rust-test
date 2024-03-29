use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use chrono::Utc;
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde::Serialize;
use serde_json::Value;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct ResponseData {
    current_time: String,
}

pub(crate) async fn my_handler(
    _event: LambdaEvent<Value>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let current_time = Utc::now().to_rfc2822();

    let current_time_data = serde_json::to_string(&ResponseData { current_time }).unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(current_time_data)),
        is_base64_encoded: false,
    };

    Ok(resp)
}
