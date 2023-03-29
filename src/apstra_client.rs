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
    token: Option<String>,
    tokened_headers: Option<reqwest::header::HeaderMap>,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tokened_headers_string: String = match &self.tokened_headers {
            Some(t) => format!("{:#?}", t),
            None => String::from("None"),
        };
        write!(
            f, 
            "( client: {}, server: {}, token: {}, headers: {:#?} )", 
            type_of(&self.client), 
            self.server, 
            self.token.clone().unwrap_or(String::from("None")), 
            tokened_headers_string
        )
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
    pub fn new(server: &String) -> Self {
        info!(target: "Client::new()", "begin: server = {}", server);
        // let my_client: reqwest::Client;
        let my_client = match reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build() {
                    Ok(t) => t,
                    Err(_e) => reqwest::Client::new(),
                };
        // TODO: fix Err(e)


        let mut client = Self {
            // client: reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap(),
            client: my_client,
            server: String::from(server),
            token: None,
            tokened_headers: None,
        };
        error!(target: "Client::new()", "end: client = {}", client);
        client

    }




    // fn get_token_header(&self) -> reqwest::header::HeaderMap {
    //     let mut headers = reqwest::header::HeaderMap::new();
    //     let token = self.token.clone().unwrap();
    //     let token_header_value = reqwest::header::HeaderValue::from_str(&token).unwrap();
    //     headers.insert("X-Auth-Token", token_header_value);
    //     headers
    // }

    // authenticate and update token and tokened_headers
    pub async fn authenticate(&mut self, login_data: &LoginData) -> Result<(), reqwest::Error> {
        let target_log = "Client::authenticate";
        info!(target: target_log, "begin with client = {self}");

        let url = self.build_url(String::from("/api/aaa/login"));
        let resp = self.client
            .post(url)
            // .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&login_data)
            .send()
            .await?
            .json::<LoginToken>()
            .await?;
        // debug!(target: target_log, "resp ={:?}", resp);
        self.token = Some(resp.token);
        debug!(target: target_log, "end: self = {}",self);

        let token_header_value = reqwest::header::HeaderValue::from_str(self.token.as_ref().unwrap()).unwrap();

        let mut tokened_headers = reqwest::header::HeaderMap::new();
        tokened_headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
        tokened_headers.insert("AuthToken", token_header_value);
        self.tokened_headers = Some(tokened_headers);
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
            // .headers(self.tokened_headers.as_ref().unwrap() clone())
            .headers(self.tokened_headers.as_ref().unwrap().clone())
            .send()
            .await?
            .text()
            .await?;
        debug!(target: target_log, "end: resp = {:#?}", resp);
        Ok(resp)
    }
    

    // getJson from url
    // take url as String to allow for query parameters
    pub async fn getJson<T>(&self, url: String) -> Result<T, reqwest::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let target_log = "Client::getJson";
        info!(target: target_log, "begin with client = {self}");
        let url = self.build_url(url);
        let resp = self.client
            .get(url)
            .headers(self.tokened_headers.as_ref().unwrap().clone())
            .send()
            .await?
            .json::<T>()
            .await?;
        // debug!(target: target_log, "end: resp = {:#?}", resp);
        Ok(resp)
    }
 
}

