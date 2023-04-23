use anyhow::Result;
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};
use serde::Deserialize;
use word_puzzle_generator::place_words;

#[derive(Deserialize)]
struct GeneratorOptions {
    size: usize,
    words: Vec<String>,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_word_puzzle_spin(req: Request) -> Result<Response> {
	let mut router = Router::new();

	router.get("/", |_req, _params| Ok(http::Response::builder()
		.status(http::StatusCode::OK)
		.body(Some("Hello from spin!".into()))?));
	router.post("/generate", |req, _params| {
		let body = req.body().as_ref().unwrap();
    let options: GeneratorOptions = serde_json::from_str(std::str::from_utf8(body.as_ref()).unwrap()).unwrap();

		if options.size > 20 {
			return Ok(http::Response::builder().status(http::StatusCode::BAD_REQUEST).body(None)?);
		}

		let grid = place_words(&options.words, options.size).unwrap();
		let response = serde_json::to_string_pretty(&grid
					.iter()
					.map(|row| row.iter().collect::<String>())
					.collect::<Vec<String>>())?.as_bytes().to_vec();
		return Ok(http::Response::builder()
			.status(http::StatusCode::OK)
			.body(Some(response.into()))?)
	});

	router.handle(req)
}
