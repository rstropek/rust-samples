use anyhow::Result;
use http::response::Builder;
use spin_sdk::{
    http::{Request, Response, Params},
    http_component, http_router,
};
use word_puzzle_generator::{place_words, GeneratorOptions};

#[http_component]
fn handle_word_puzzle_spin(req: Request) -> Result<Response> {

		if req.method() == http::Method::OPTIONS {
				return Ok(http::Response::builder()
						.status(http::StatusCode::OK)
						.header("Access-Control-Allow-Origin", "*")
						.header("Access-Control-Allow-Methods", "*")
						.header("Access-Control-Allow-Headers", "*")
						.body(None)?);
		}

    let router = http_router! {
    	GET "/" => |_req, _params| {
        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some("Hello from spin!".into()))?)
    	},

			POST "/generate" => generate_puzzle
		};

    router.handle(req)
}

fn generate_puzzle(req: Request, _params: Params) -> Result<Response> {
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
				.with_cors()
        .header("Content-Type", "application/json")
				.body(Some(response.into()))?)
}

trait WithCors {
	fn with_cors(self) -> Self;
}

impl WithCors for Builder {
	fn with_cors(self) -> Self {
		self.header("Access-Control-Allow-Origin", "*")
				.header("Access-Control-Allow-Methods", "*")
				.header("Access-Control-Allow-Headers", "*")
	}
}
