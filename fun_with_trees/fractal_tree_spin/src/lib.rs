use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod parameters;

#[http_component]
fn fractal_tree_spin(_req: Request) -> Result<Response> {
    let params = parameters::extract_parameters(&_req);

    let buffer = fractal_tree::tree_svg(params.seed, params.depth, params.trunk_length);

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "image/svg+xml")
        .body(Some(buffer.into()))?)
}
