// use reqwest;
use log::{debug, info, warn, error};
use env_logger;

use chrono::Local;
use std::io::Write;

use std::collections::HashMap;

mod apstra_client;


#[tokio::main]
async fn main() { 
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{} {}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .write_style(env_logger::WriteStyle::Always)
        .init();   

    info!("Starting....");
    let mut client = apstra_client::Client::new(&String::from("https://10.85.192.50"));

    let login_data = apstra_client::LoginData {
        username: String::from("admin"),
        password: String::from("zaq1@WSXcde3$RFV"),
    };
    match client.authenticate(&login_data).await {
        Ok(_t) => debug!("auth done"),
        Err(_e) => error!("auth err"),
    }
    warn!("client = {}", client);
    let result = client
        .getText(String::from("/api/blueprints"))
        // .header(reqwest::header::ACCEPT, "application/json")
        // .send()
        .await;
    match result {
        Ok(t) => debug!("result = {:#?}", t),
        Err(e) => error!("{:?}", e),
    }
//     error!("blueprints result = {:#?}", result);
}

