use std::{collections::HashMap, io::Read};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
struct Parameters {
    args: Vec<String>,
    vars: HashMap<String, String>,
    body: Option<Value>,
}

#[allow(dead_code)]
fn main() {
    println!("Content-Type: text/html");
    println!();
    println!("<html>
        <body>
            <ul>
                <li><a href=\"level3/inline?foo=bar&answer=42\">Inline template</a></li>
                <li><a href=\"level3/file\">Template from file</a></li>
            </ul>
        </body>
    </html>");
}

fn get_parameters() -> Parameters {
    let mut body= String::new();
    let mut param_body = None;
    if std::env::var("CONTENT_LENGTH").unwrap().parse::<usize>().unwrap() != 0
        && std::io::stdin().read_to_string(&mut body).unwrap() > 0
        && !body.trim().is_empty() {
        if let Ok(body_json) = serde_json::from_str::<Value>(&body) {
            param_body = Some(body_json);
        }
    }

    Parameters{
        args: std::env::args().collect(),
        vars: std::env::vars().collect(),
        body: param_body
    }
}

#[no_mangle]
pub fn inline() {
    println!("Content-Type: text/html\n");

    let data = get_parameters();

    let template = "<html>
        <body>
            <ul>
                {{#each args}}
                    <li>{{this}}</li>
                {{/each}}
            </ul>
        </body>
    </html>";

    let mut reg = Handlebars::new();
    reg.register_template_string("template", template).unwrap();
    println!("{}", reg.render("template", &json!(data)).unwrap());
}

#[no_mangle]
pub fn template_file() {
    println!("Content-Type: text/html\n");

    let data = get_parameters();

    let mut reg = Handlebars::new();
    let template_path = format!("{}/template.hbs", std::env::var("TEMPLATE_PATH").unwrap());
    reg.register_template_file("template", template_path).unwrap();
    println!("{}", reg.render("template", &json!(data)).unwrap());
}

#[no_mangle]
pub fn _routes() {
    // Learning: How to declare subroutes (https://github.com/deislabs/wagi/blob/main/docs/writing_modules.md#advanced-declaring-sub-routes-in-the-module)
    println!("/inline inline");
    println!("/file template_file");
}