use std::collections::HashMap;
use log::{debug, info, warn, error};

pub struct Client {
    // client: reqwest::Client,
    server: String,
    token: String,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( server: {}, token: {})", self.server, self.token)
    }
}


impl Client {
    pub fn new(server: &String) -> Client {
        debug!(target: "Client::new()", "begin: server = {}", server);
        let my_client = reqwest::Client::new();
        // let client = Client {
        //     // client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
        //     client: reqwest::Client::new(),
        //     server: server.to_string(),
        //     token: "".to_string(),
        // };
        // let mut auth_data = HashMap::new();
        // auth_data.insert("username", "admin");
        // auth_data.insert("password", "admin");

        // let res = client.client.post(format!("{}{}", server, "/api/aaa/login"))
        //     // format!("https://{server}/api/aaa/login"))
        //     .json(&auth_data)
        //     .header(reqwest::header::CONTENT_TYPE, "application/json")
        //     // .header("AuthToken", self.token)
        //     .send()
        //     .await?
        //     .text()
        //     .await?;
        // println!("Client::new({})", server);
        let response = Client::authenticate(server);
        // future::executor::block_on(response);
        // match response.status() {
        //     reqwest::StatusCode::OK => {
        //         println!("Success!");
        //     }
        //     _ => {
        //         println!("statuscode = {}", response.status());
        //     }
        // }

        let client = Client {
            // client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
            // client: my_client,
            server: server.to_string(),
            token: "".to_string(),
        };
        debug!(target: "Client::new()", "ending: client = {}", client);
        client

    }


    async fn authenticate(server: &String) -> Result<(), reqwest::Error> {
        info!(target: "Client::authenticate", "begin with server = {server}");
        // println!("authenticate({})", server);
        // let mut auth_data = HashMap::new();
        // auth_data.insert("username", "admin");
        // auth_data.insert("password", "admin");
        // let auth_body = serde_json::to_string(&auth_data).unwrap();

        // let mut headers = reqwest::header::HeaderMap::new();
        // headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

        let token_json: serde_json::Value = reqwest::Client::new()
            .post(format!("{}{}", server, "/api/aaa/login"))
            // .header(reqwest::header::CONTENT_TYPE, "application/json")
            // .header("AuthToken", self.token)
            .json(&serde_json::json!({
                "username": "admin",
                "passowrd": "admin"
            }))
            // .body(auth_body)
            .send()
            .await.unwrap()
            .json()
            .await.unwrap();
        println!("token_json = {:#?}", token_json);
        Ok(())
        // "TO-BE-FIXED-token-string".to_string()

        // let _response = client
        //     .post(format!("{}{}", server, "/api/aaa/login"))
        //     .header(reqwest::header::CONTENT_TYPE, "application/json")
        //     // .header("AuthToken", self.token)
        //     .body(auth_body)
        //     .send()
        //     .await
        //     .unwrap();
        // "token is hear".to_string()
        // // response
    }

    fn build_url(&self, url: String) -> String {
        debug!(target: "Client::build_url()",  "begin with server = {}, url = {url}...", self.server);
        format!("{}{}", self.server, url)
    }

    // pub fn client(&self) -> &reqwest::Client {
    //     &self.client
    // }

    // pub fn get(&self, url: String) -> reqwest::RequestBuilder {
    pub async fn get(&self, url: String) -> Result<(), Box<dyn std::error::Error>> {
        let built_url = self.build_url(url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

        let resp = reqwest::get(built_url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())

        // let client = reqwest::Client::builder()
        //     .default_headers(headers)
        //     .danger_accept_invalid_certs(true)
        //     .build();
        // let response = client
        //     .get(built_url)
        //     // .header(reqwest::header::CONTENT_TYPE, "application/json")
        //     .send()
        //     .await;
        //     // .unwrap();
        // println!(":Success! {:?}", response);
        // match response.status() {
        //     reqwest::StatusCode::OK => {
        //         println!("Success! {:?}", response);
        //     }
        //     reqwest::StatusCode::UNAUTHORIZED => {
        //         println!("Need to grab a new token");
        //     }
        //     other => {
        //         panic!("Uh oh! Something unexpected happened: {:?}", other);
        //     }
        // }
    }
}

