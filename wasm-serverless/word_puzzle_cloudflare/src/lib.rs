use word_puzzle_generator::{place_words, GeneratorOptions};
use worker::*;

mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/generate", generate_puzzle)
        .run(req, env)
        .await
}

async fn generate_puzzle(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    match req.json().await {
        Err(_) => Response::error("Bad Request", 400),
        Ok(options) => {
            let options: GeneratorOptions = options;
            if options.size > 20 {
                return Response::error("Bad Request", 400);
            }

            let puzzle = place_words(options);
            Response::from_json(&puzzle)
        }
    }
}
