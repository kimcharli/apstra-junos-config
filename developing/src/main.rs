// use reqwest;
use log::{debug, info, warn, error};
use env_logger;

mod apstra_client;


#[tokio::main]
async fn main() { 
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Always)
        .init();   

    info!("Starting....");
    let client = apstra_client::Client::new(&"https://10.85.192.50".to_string());
    match client.do_authenticate().await {
        Ok(_t) => debug!("auth done"),
        Err(_e) => error!("auth err"),
    }
    warn!("client = {}", client);
    let result = client
        .get("/api/blueprints".to_string())
        // .header(reqwest::header::ACCEPT, "application/json")
        // .send()
        .await;
    error!("blueprints result = {:#?}", result);
}

