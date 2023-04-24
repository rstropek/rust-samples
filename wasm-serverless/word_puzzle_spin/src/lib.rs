use anyhow::Result;
use routefinder::Captures;
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};
use word_puzzle_generator::{place_words, GeneratorOptions};

#[http_component]
fn handle_word_puzzle_spin(req: Request) -> Result<Response> {
    let mut router = Router::new();

    router.get("/", |_req, _params| {
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some("Hello from spin!".into()))?)
    });

    router.post("/generate", generate_puzzle);

    router.handle(req)
}

fn generate_puzzle(req: Request, _params: Captures) -> Result<Response> {
    let body = req.body().as_ref().unwrap();
    let options: GeneratorOptions = serde_json::from_str(std::str::from_utf8(body.as_ref())?)?;

    if options.size > 20 {
        return Ok(http::Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .body(None)?);
    }

    let puzzle = place_words(options);
    let response = serde_json::to_string_pretty(&puzzle)?.as_bytes().to_vec();
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Some(response.into()))?)
}
