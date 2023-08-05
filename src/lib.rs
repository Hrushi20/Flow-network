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

async fn handler(headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let lottery:Lottery = match serde_json::from_slice(&_body) {
        Ok(res) => res,
        Err(err) => {
            let resp = format!("Invalid body format. Check the readme. Err:{}",err);
            send_response(
                200,
                vec![(String::from("content-type"), String::from("text/html"))],
                resp.as_bytes().to_vec(),
            );
            return;
        }
    };

    let resp;
    let guess = lottery.guess;
    if guess < 0 || guess > 3 {
       resp = format!("Number entered {} not in range [0-3] inclusive",guess);
        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            resp.as_bytes().to_vec()
        );
        return;
    }
    let v = vec![0,1, 2, 3];
    let lucky_num = fastrand::usize(..v.len()) as i32;

    if lucky_num == guess {
       resp = format!("Congrats!!! You won the lottery!!!");
        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            resp.as_bytes().to_vec()
        );
        return;
    }

    resp = format!("You entered: {}. The lucky number is:{}. Try again to improve you luck",guess,lucky_num);
    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );
}