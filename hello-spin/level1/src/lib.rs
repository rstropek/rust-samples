use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn ping(_request: Request) -> Result<Response> {
    Ok(http::Response::builder()
        .status(200)
        .body(Some("pong".into()))?)
}
