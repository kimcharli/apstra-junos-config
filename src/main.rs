use clap::Parser;
use anyhow::{Context, Result};

/// Retrive the device configurations from Apstra 
#[derive(Parser)]
struct Cli {
    /// The Apstar URL to contact
    apstra_url: String,
    /// The path to the folder to save configuraitons
    destination_folder: std::path::PathBuf,
}


// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.destination_folder;
    // // let content = std::fs::read_to_string(&args.destination_folder).expect("could not read file");
    // let result = std::fs::read_to_string(&args.destination_folder);
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with {}, just exit here", error ); }
    // };
    // let content = std::fs::read_to_string(&args.destination_folder)?;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.to_string_lossy()))?;

    println!("file content: {}", content);
    // println!("url: {}, path: {}", args.apstra_url, content);
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    Ok(())
}
