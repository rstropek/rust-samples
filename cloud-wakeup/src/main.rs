extern crate clap;
extern crate oauth2;
extern crate url;
extern crate base64;
extern crate serde_json;

use clap::{App, Arg};
use oauth2::prelude::*;
use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, Scope, TokenResponse};
use oauth2::basic::BasicClient;
use url::Url;
use serde_json::to_vec;
use std::str;

fn main() {
    let matches = App::new("Cloud Wakeup")
        .version("0.1")
        .author("Rainer Stropek <rainer@software-architects.at>")
        .arg(
            Arg::with_name("sb-namespace")
                .short("n")
                .long("servicebus-namespace")
                .help("Sets Azure Service Bus Namespace")
                .env("CW_SB_NAMESPACE")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("queue")
                .short("q")
                .long("queue-name")
                .help("Sets the name of the Service Bus queue")
                .env("CW_QUEUE_NAME")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client-id")
                .short("c")
                .long("client-id")
                .help("Sets the AAD client (=app) id")
                .env("CW_CLIENT_ID")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client-secret")
                .short("s")
                .long("client-secret")
                .help("Sets the secret for the AAD client (=app) id")
                .env("CW_CLIENT_SECRET")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("aad-tenant")
                .short("t")
                .long("aad-tenant")
                .help("Sets the Azure AD tenant")
                .env("CW_AAD_TENANT")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let client_id = matches.value_of("client-id").unwrap().to_string();
    let client_secret = matches.value_of("client-secret").unwrap().to_string();
    let aad_tenant = matches.value_of("aad-tenant").unwrap();
    let authorize = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", aad_tenant);
    let token = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", aad_tenant);

    let client =
        BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(Url::parse(&authorize).unwrap()), Some(TokenUrl::new(Url::parse(&token).unwrap()))
        )
        .add_scope(Scope::new("https://servicebus.azure.net/.default".to_string()));

    let cred = client.exchange_client_credentials().unwrap();
    let token_result = TokenResponse::access_token(&cred);
    let c2 = to_vec(token_result).unwrap();
    let c2_string = str::from_utf8(&c2[1..c2.len() - 1]).unwrap();

    println!("{}", c2_string);
}
