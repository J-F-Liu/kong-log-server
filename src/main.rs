#![feature(async_await, futures_api, await_macro)]

use std::env;
use tide::middleware::DefaultHeaders;
use tide::{Context, EndpointResult, error::ResultExt, response, App};
use tide::http::StatusCode;
use std::sync::Mutex;
use serde_json::{json, Value, Error};

#[derive(Default)]
struct Database {
}

impl Database {
}

// Get the PORT number from the env variable
fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8181)
}

fn main() {
    let mut app = tide::App::new(());

    app.middleware(
        DefaultHeaders::new()
            .header("server", "Tide 0.2.0"),
    );

    app.at("/").get(async move |_| "Kong Log Server");
    app.at("/log").post(save_log);

    app.serve(format!("0.0.0.0:{}", get_server_port()))
      .expect("Start server failed.");
}

async fn save_log(mut cx: Context<()>) -> EndpointResult<()> {
  let log = await!(cx.body_string()).client_err()?;
  let parsed: Value = serde_json::from_str(&log).unwrap_or(json!({}));
  if parsed["upstream_uri"] != json!("/metrics") {
     println!("{}", log);
  }
  Ok(())
}
