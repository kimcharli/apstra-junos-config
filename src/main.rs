use clap::Parser;
// use anyhow::{Context, Result};
use anyhow::{Result};
// use url::{Url, ParseError};
// use url::{Url};

use reqwest::{Client, header::CONTENT_TYPE};
use std::{env, collections::HashMap};

/// Retrive the device configurations from Apstra 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Apstar Server
    #[arg(short, long)]
    #[clap(default_value="10.85.192.62")]
    server: String,

    /// User name to the Apstra server
    #[arg(short, long)]
    #[clap(default_value="admin")]
    user: String,

    /// Password of the user
    #[arg(short, long)]
    #[clap(default_value="admin")]
    password: String,

    /// Blueprint name
    #[arg(short, long)]
    #[clap(default_value="TJC")]
    blueprint: String,

    /// Path to the folder to save configuraitons
    #[arg(short, long)]
    #[clap(default_value="work/test1")]
    output_folder: std::path::PathBuf,

    /// Proxy URL
    #[arg(short, long)]
    #[clap(default_value="")]
    web_proxy: String,
}

struct CkApstraServer<'a> {
    server: String,
    // port: u16,
    auth_data: HashMap<&'a str, String>,
    // user: String,
    // password: String,
    blueprint: String,
    client: reqwest::Client,
    token: String,    
}

impl CkApstraServer<'_> {
    async fn new(server: String, user: String, password: String, blueprint: String) -> Result<CkApstraServer<'static>, reqwest::Error> {
        // let u16_port = port.parse::<u16>().unwrap();
        let mut auth_data = HashMap::new();
        auth_data.insert("username", user);
        auth_data.insert("password", password);

        let client = reqwest::Client::new();
        // let url = "https://{}/api/aaa/login", server
        let res = client.post(format!("https://{server}/api/aaa/login"))
            .json(&auth_data)
            .header(CONTENT_TYPE, "application/json")
            // .header("AuthToken", self.token)
            .send()
            .await?
            .text()
            .await?;

        println!("res = {res}");

        Ok(CkApstraServer { server: server, auth_data: auth_data, blueprint: blueprint, client: client, token: res})
    } 

    fn print_token(self) {
        println!("token = {}", self.token);
    }
}


// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}", args);

    let args = Arguments::parse();
    println!("args = {:?}", args);
    let path = &args.output_folder;
    // // let content = std::fs::read_to_string(&args.destination_folder).expect("could not read file");
    // let result = std::fs::read_to_string(&args.destination_folder);
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with {}, just exit here", error ); }
    // };
    // let content = std::fs::read_to_string(&args.destination_folder)?;
    // let content = std::fs::read_to_string(path)
    //     .with_context(|| format!("could not read file `{}`", path.to_string_lossy()))?;

    // println!("file content: {}", content);
    // println!("url: {}, path: {}", args.apstra_url, content);
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    std::fs::create_dir_all(path)?;
    // let apstra_url = Url::parse(&args.host)?;
    // println!("apstra_url scheme = {}, host = {}, port = {}, user = {}", apstra_url.scheme(), apstra_url.host(), apstra_url.port(), apstra_url.username());
    // println!("apstra_url scheme = {}, user = {}", apstra_url.scheme(), apstra_url.username());

    let server = CkApstraServer::new( args.server, args.user, args.password, args.blueprint );
    match server {
        Ok(myserver) => myserver.print_token(),
        Err(err) => println!("Error: {}", err)
    };
    
    // server.print_token();

    Ok(())
}


