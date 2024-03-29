use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{
    ApiGatewayProxyRequest, ApiGatewayProxyResponse, ApiGatewayV2httpRequest,
};
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde::de::IntoDeserializer;
use serde::Deserialize;
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

pub(crate) async fn my_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    // let payload = serde_json::from_value(event.payload);

    let (event, _context) = event.into_parts();
    let path = event.path.unwrap();

    let message = format!("Hello world from {}", path);

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(message)),
        is_base64_encoded: false,
    };

    Ok(resp)
}
