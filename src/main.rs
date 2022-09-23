mod filters;
mod mailer;
mod routes;

use crate::mailer::SmtpConnectionOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use tokio::fs;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RestApiConfig {
    pub api_keys: HashMap<String, SmtpConnectionOptions>,
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "rest_smtp=info");
    }
    pretty_env_logger::init();

    let api_config_data = fs::read_to_string("./api_config.json")
        .await
        .expect("Could not read file api_config.json file");
    let api_config: RestApiConfig =
        serde_json::from_str(&api_config_data).expect("Could not parse api_config.json contents");

    let api_routes = filters::rest_smtp_api(api_config);
    let api_routes = api_routes.with(warp::log("rest_smtp"));

    warp::serve(api_routes).run(([127, 0, 0, 1], 9002)).await;
}
