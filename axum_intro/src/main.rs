use axum::{routing::get, Router, extract::{Path, Query}, Json, response::IntoResponse, http::StatusCode};
use rand::Rng;
use serde::{Serialize, Deserialize};

// Note that Axum uses tokio as its runtime, so we need to use the #[tokio::main]
#[tokio::main]
async fn main() {
    // Learn more about routing at https://docs.rs/axum/latest/axum/index.html#routing
    let app = Router::new()
        .route("/", get(hello_world).post(|| async { "Hello, World ✉️!"}))
        // Let's add additional routes. Note that we can structure complex
        // routing hierarchies using methods like merge and nest.
        .merge(pingpong())
        .nest("/kingkong", kingkong())
        .route("/greet/:name", get(greet_path))
        .route("/greet", get(greet_query).post(greet_body))
        .route("/lookup/:number", get(look_it_up))
        .route("/random", get(return_something_random));

    // Run the server on port 3000. In practice, make settings like that
    // configurable (e.g. using clap).
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Let's move the code of the handler into a dedicated function.
// Don't forget to change the router so that it points to the fuction.
async fn hello_world() -> &'static str {
    // A handler is a function that takes 0..n extractors (see later)
    // and returns something that implements IntoResponse.
    "Hello, World!"
}

// Two functions that return a router. This is very useful in larger applications
// with lots of routes.
fn pingpong() -> Router {
    Router::new().route("/ping", get(|| async { "pong" }))
}

fn kingkong() -> Router {
    async fn king() -> &'static str {
        "Kong"
    }

    Router::new().route("/king", get(king))
}

// Path is an "Extractor". Extractors are used to extract data from the request.
// .route("/greet/:name", get(greet_path))
async fn greet_path(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct QueryParameters {
    salutation: Option<String>,
    name: Option<String>,
}

// Query is also an "Extractor".
// .route("/greet", get(greet_query))
async fn greet_query(Query(params): Query<QueryParameters>) -> String {
    let salutation = params.salutation.unwrap_or_else(|| "Hello".to_string());
    let name = params.name.unwrap_or_else(|| "World".to_string());
    format!("{}, {}!", salutation, name)
}

// Json is the third commonly used extractor.
// .route("/greet", get(greet_query).post(greet_body))
async fn greet_body(Json(params): Json<QueryParameters>) -> String {
    let salutation = params.salutation.unwrap_or_else(|| "Hello".to_string());
    let name = params.name.unwrap_or_else(|| "World".to_string());
    format!("{}, {}!", salutation, name)
}

// NOTE: You can learn all details about extractor at
// https://docs.rs/axum/latest/axum/extract/index.html#defining-custom-extractors

#[derive(Serialize)]
struct LookupResult {
    number: i32,
    found: bool,
}

// A lot of types implement IntoResponse, including tuples.
// Read more at https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html
// .route("/lookup", get(look_it_up))
async fn look_it_up(Path(number): Path<i32>) -> impl IntoResponse {
    // Let's say that only odd numbers are "found"
    match number % 2 {
        1 => (StatusCode::OK, Json(LookupResult { number, found: true })),
        _ => (StatusCode::NOT_FOUND, Json(LookupResult { number, found: false })),
    }
}

// The easiest way to return different data types from a handler
// is to convert them into Response, which implements IntoRespose.
// .route("/random", get(return_something_random))
async fn return_something_random() -> impl IntoResponse {
    // Generate random number between 0 and 2 (including)
    match rand::thread_rng().gen_range(0..3) {
        0 => "Hello, World!".into_response(),
        1 => StatusCode::NOT_IMPLEMENTED.into_response(),
        _ => Json(42).into_response(),
    }
}
