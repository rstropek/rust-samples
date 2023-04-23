use serde::Deserialize;
use serde_json::json;
use word_puzzle_generator::place_words;
use worker::*;

mod utils;

#[derive(Deserialize)]
struct GeneratorOptions {
    size: usize,
    words: Vec<String>,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .post_async("/generate", |mut req, _ctx| async move {
            match req.json().await {
                Err(_) => Response::error("Bad Request", 400),
                Ok(options) => {
                    let options: GeneratorOptions = options;
                    if options.size > 20 {
                        return Response::error("Bad Request", 400);
                    }

                    match place_words(&options.words, options.size) {
                        Err(unplaced_words) => Ok(Response::from_json(&json!({
                            "error": "Unable to place all words",
                            "unplaced_words": unplaced_words
                        }))
                        .unwrap()
                        .with_status(500)),
                        Ok(grid) => Response::from_json(&json!(grid
                            .iter()
                            .map(|row| row.iter().collect::<String>())
                            .collect::<Vec<String>>())),
                    }
                }
            }
        })
        .run(req, env)
        .await
}
