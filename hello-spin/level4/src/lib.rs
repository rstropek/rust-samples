use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use serde::{Serialize};

#[derive(Serialize)]
struct SessionListPayload {
    culture: String,
    #[serde(rename = "sessions")]
    event_id: String,
}

#[http_component]
fn get_session(_request: Request) -> Result<Response> {
    let payload = SessionListPayload {
        culture: "nl-NL".to_string(),
        event_id: "c8330994-c8bb-47f8-bc9b-c5776656470e".to_string(),
    };

    let pl = serde_json::to_string(&payload)?;
    let pl = pl.as_bytes();

    let res = spin_sdk::http::send(
        http::Request::builder()
            .method("POST")
            .uri("https://www.techorama.nl/umbraco/api/schedules/get")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(Some(bytes::Bytes::copy_from_slice(pl)))?,
    )?;

    Ok(res)
}
