use std::collections::HashMap;
use std::any::type_name;
use log::{debug, info, warn, error};


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


    pub async fn do_authenticate(&self) -> Result<(), reqwest::Error> {
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
            .json(&serde_json::json!({
                "username": "admin",
                "passowrd": "admin"
            }))
            // .body(auth_body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        debug!(target: target_log, "end: resp ={:?}, self = {}", resp, self);
        Ok(())

    }

    fn build_url(&self, url: String) -> String {
        warn!(target: "Client::build_url()",  "begin with server = {}, url = {url}...", self.server);
        format!("{}{}", self.server, url)
    }

    // pub fn client(&self) -> &reqwest::Client {
    //     &self.client
    // }

    pub async fn get(&self, url: String) -> Result<(), Box<dyn std::error::Error>> {
        let target_log = "Client::get()";
        debug!(target: target_log, "begin...");
        let built_url = self.build_url(url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

        let resp = reqwest::get(built_url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
}

