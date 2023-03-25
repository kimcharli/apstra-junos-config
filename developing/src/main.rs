// use reqwest;
use log::{debug, info, warn, error};
use env_logger;

mod apstra_client;

// struct Client {
//     client: reqwest::Client,
//     server: String,
//     token: String,
// }

// impl Client {
//     pub fn new(server: String) -> Client {
//         let mut client = Client {
//             client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
//             server: server,
//             token: "".to_string(),
//         };
//         // let mut auth_data = std::collections::HashMap::new();
//         // auth_data.insert("username", "admin");
//         // auth_data.insert("password", "admin");

//         // let res = client.client.post(format!("{}{}", server, "/api/aaa/login"))
//         //     // format!("https://{server}/api/aaa/login"))
//         //     .json(&auth_data)
//         //     .header(reqwest::header::CONTENT_TYPE, "application/json")
//         //     // .header("AuthToken", self.token)
//         //     .send()
//         //     .await?
//         //     .text()
//         //     .await?;
//         let response = authenticate(server);
//         future::executor::block_on(response);

//         client
//     }

//     async fn authenticate(server: String) ->  {
//         let mut auth_data = std::collections::HashMap::new();
//         auth_data.insert("username", "admin");
//         auth_data.insert("password", "admin");

//         let response = client.client.post(format!("{}{}", server, "/api/aaa/login"))
//             // format!("https://{server}/api/aaa/login"))
//             .json(&auth_data)
//             .header(reqwest::header::CONTENT_TYPE, "application/json")
//             // .header("AuthToken", self.token)
//             .send()
//             .await?
//             .text()
//             .await?;
//         response
//     }

//     fn build_url(&self, url: String) -> String {
//         println!("url = {}{}", self.server, url);
//         format!("{}{}", self.server, url)
//     }

//     pub fn client(&self) -> &reqwest::Client {
//         &self.client
//     }

//     // pub fn get(&self, url: String) -> reqwest::RequestBuilder {
//     pub fn get(&self, url: String) -> reqwest::RequestBuilder {
//         let built_url = self.build_url(url);
//         self.client
//             .get(built_url)
//             .header(reqwest::header::CONTENT_TYPE, "application/json")
//         }
// }


#[tokio::main]
async fn main() { 
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Always)
        .init();   

    info!("Starting....");
    // println!("Hello, world!");
    // let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    let client = apstra_client::Client::new(&"https://10.85.192.50".to_string());
    match client.do_authenticate().await {
        Ok(_) => debug!("auth done"),
        Err(e) => error!("auth err"),
    }
    warn!("client = {}", client);
    // println!("Type of client = {}", client.type_name());
    // let result = client.client()
    let result = client
        .get("/api/blueprints".to_string())
        // .header(reqwest::header::ACCEPT, "application/json")
        // .send()
        .await;
    error!("blueprints result = {:#?}", result);
    // match result {
    //     Err(e) => println!("Error: {:?}", e),
    //     // Ok(v) => println!("Body: {:?}", v)
    //     Ok(v) => match v.status() {
    //         reqwest::StatusCode::OK => { println!("Success! {:?}", reqwest::StatusCode::OK); },
    //         reqwest::StatusCode::UNAUTHORIZED => { println!("Need to grab a new token"); },
    //         _ => { panic!("Uh oh! Something unexpected happened."); },
    //     }
    // }
}

