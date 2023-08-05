use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Lottery {
    guess: i32
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let msg = qry.get("msg").unwrap();
    // let msg = String::from_utf8(body).unwrap_or("".to_string());

    let lottery:Lottery = serde_json::from_slice(&_body).unwrap();

    let resp = format!("Welcome to flows.network.\nYou just said: '{}' '{}'.\nLearn more at: https://github.com/flows-network/hello-world\n", msg,lottery.guess);
    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );
}