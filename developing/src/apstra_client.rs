use std::collections::HashMap;
use std::any::type_name;
use log::{debug, info, warn, error};
use serde::{Deserialize, Serialize};


fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// #[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    server: String,
    token: String,
    tokened_headers: reqwest::header::HeaderMap,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( client: {}, server: {}, token: {})", type_of(&self.client), self.server, self.token)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LoginToken {
    id: String,
    token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}


impl Client {
    pub fn new(server: &String) -> Client {
        info!(target: "Client::new()", "begin: server = {}", server);
        // let my_client: reqwest::Client;
        let my_client = match reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build() {
                    Ok(t) => t,
                    Err(_e) => reqwest::Client::new(),
                };
        // TODO: fix Err(e)


        let mut client = Client {
            // client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
            client: my_client,
            server: server.to_string(),
            token: "".to_string(),
            tokened_headers: reqwest::header::HeaderMap::new(),
        };
        error!(target: "Client::new()", "end: client = {}", client);
        client

    }

    // authenticate and update token and tokened_headers
    pub async fn authenticate(&mut self, login_data: &LoginData) -> Result<(), reqwest::Error> {
        let target_log = "Client::authenticate";
        info!(target: target_log, "begin with client = {self}");

        // let login_data = LoginData {
        //     username: "admin".to_string(),
        //     password: "zaq1@WSXcde3$RFV".to_string(),
        // };

        let url = self.build_url("/api/aaa/login".to_string());
        let resp = self.client
            .post(url)
            // .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&login_data)
            .send()
            .await?
            .json::<LoginToken>()
            .await?;
        // debug!(target: target_log, "resp ={:?}", resp);
        self.token = resp.token;
        debug!(target: target_log, "end: self = {}",self);

        let token_header_value = reqwest::header::HeaderValue::from_str(&self.token).unwrap();

        self.tokened_headers = reqwest::header::HeaderMap::new();
        self.tokened_headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
        self.tokened_headers.insert("AuthToken", token_header_value);
        debug!(target: target_log, "headers = {:#?}", self.tokened_headers);

        Ok(())

    }

    fn build_url(&self, url: String) -> String {
        warn!(target: "Client::build_url()",  "begin with server = {}, url = {url}...", self.server);
        format!("{}{}", self.server, url)
    }

    // get String output from url
    // take url as String to allow for query parameters
    pub async fn getText(&self, url: String) -> Result<String, reqwest::Error> {
        let target_log = "Client::getText()";
        debug!(target: target_log, "begin...");
        let built_url = self.build_url(url);

        let resp = self.client
            .get(built_url)
            .headers(self.tokened_headers.clone())
            .send()
            .await?
            .text()
            .await?;
        debug!(target: target_log, "end: resp = {:#?}", resp);
        Ok(resp)
    }
    

    pub async fn getJson(&self, url: String) -> Result<HashMap<String, String>, reqwest::Error> {
        let target_log = "Client::get()";
        debug!(target: target_log, "begin...");
        let built_url = self.build_url(url);
        let token_header_value = reqwest::header::HeaderValue::from_str(&self.token).unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
        headers.insert("AuthToken", token_header_value);
        debug!(target: target_log, "headers = {:#?}", headers);

        let resp = self.client
            .get(built_url)
            .headers(headers)
            .send()
            // .await?
            // .json::<HashMap<String, String>>()
            .await?;

        // q: how to clone above resp ?
    
        // println!("{:#?}", resp);
        // println!("{:#?}", resp.text().await);

        let json_data = resp.json::<HashMap<String, String>>().await;
        match json_data {
            Ok(t) => {debug!("result(t) = {:#?}", t); Ok(t)},
            Err(e) => {error!("{:?}", e); Err(e)},
        }
        // println!("{:#?}", resp);
        // Ok(resp)
    }
}

