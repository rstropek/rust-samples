use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::{str, collections::HashMap};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct Parameters {
    args: Vec<String>,
    vars: HashMap<String, String>,
    body: Option<Value>,
}

fn get_parameters(request: Request) -> Parameters {
    let mut body = None;

    if str::from_utf8(request.headers().get("content-length").unwrap().as_ref()).unwrap().parse::<usize>().unwrap() != 0 {
        if let Ok(body_json) = serde_json::from_str(str::from_utf8(request.body().as_ref().unwrap().as_ref()).unwrap()) {
            body = Some(body_json);
        }
    }

    Parameters { 
        args: request.uri().query().unwrap_or("None").split('&').map(String::from).collect(),
        vars: request.headers().iter().map(|h| (String::from(h.0.as_str()), String::from(str::from_utf8(h.1.as_ref()).unwrap()))).collect(), 
        body 
    }
}

#[http_component]
fn ping(request: Request) -> Result<Response> {
    let data = get_parameters(request);

    let mut reg = Handlebars::new();
    let template_path = format!("{}/template.hbs", std::env::var("TEMPLATE_PATH").unwrap());
    reg.register_template_file("template", template_path).unwrap();

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(reg.render("template", &json!(data)).unwrap().into()))?)
}
