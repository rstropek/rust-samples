use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::str;

#[http_component]
fn ping(request: Request) -> Result<Response> {
    let protocol = format!("
### Query
{}

### Headers
{}

### Body
{}
    ", request.uri().query().unwrap_or("None")
    , request.headers().iter().map(|h| format!("{}={}", h.0.as_str(), str::from_utf8(h.1.as_ref()).unwrap())).collect::<Vec<String>>().join("\n")
    , str::from_utf8(request.body().as_ref().unwrap()).unwrap());

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(protocol.into()))?)
}
