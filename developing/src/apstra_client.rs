use std::collections::HashMap;
use std::any::type_name;
use log::{debug, info, warn, error};
use serde::{Deserialize, Serialize};


fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub struct Client {
    client: reqwest::Client,
    server: String,
    token: String,
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


        let client = Client {
            // client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
            client: my_client,
            server: server.to_string(),
            token: "".to_string(),
        };
        // client.authenticate();
        error!(target: "Client::new()", "end: client = {}", client);
        client

    }

    // fn set_token(&mut self, token: String) {
    //     self.token = token;
    // }

    pub async fn authenticate(&mut self) -> Result<(), reqwest::Error> {
        let target_log = "Client::authenticate";
        info!(target: target_log, "begin with client = {self}");
        // println!("authenticate({})", server);
        // let mut auth_data = HashMap::new();
        // auth_data.insert("username", "admin");
        // auth_data.insert("password", "admin");
        // let auth_body = serde_json::to_string(&auth_data).unwrap();

        // let mut headers = reqwest::header::HeaderMap::new();
        // headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

        let url = self.build_url("/api/aaa/login".to_string());
        let resp = self.client
            // .post(format!("{}{}", self.server, "/api/aaa/login"))
            .post(url)
            // .header(reqwest::header::CONTENT_TYPE, "application/json")
            // .header("AuthToken", self.token)
            // .json(&serde_json::json!({
            //     "username": "admin",
            //     "passowrd": "admin"
            // }))
            .body("{\"username\": \"admin\", \"password\": \"zaq1@WSXcde3$RFV\"}")
            .send()
            .await?
            .json::<LoginToken>()
            .await?;
        // self.set_token(resp.token);
        debug!(target: target_log, "resp ={:?}", resp);
        self.token = resp.token;
        debug!(target: target_log, "end: self = {}",self);
        Ok(())

    }

    fn build_url(&self, url: String) -> String {
        warn!(target: "Client::build_url()",  "begin with server = {}, url = {url}...", self.server);
        format!("{}{}", self.server, url)
    }

    // pub fn client(&self) -> &reqwest::Client {
    //     &self.client
    // }

    pub async fn get(&self, url: String) -> Result<HashMap<String, String>, reqwest::Error> {
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
        let json_data = resp.json::<HashMap<String, String>>().await;
        match json_data {
            Ok(t) => {debug!("result(t) = {:#?}", t); Ok(t)},
            Err(e) => {error!("{:?}", e); Err(e)},
        }
        // println!("{:#?}", resp);
        // Ok(resp)
    }
}

