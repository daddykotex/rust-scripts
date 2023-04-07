use clap::Parser;
use reqwest::blocking::Client;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// API key used to authenticate
    #[arg(long)]
    api_key: String,
    /// Secret used to request an Access Token
    #[arg(long)]
    api_secret: String,
}

fn main() {
    let args = Args::parse();

    println!("KEY = [{}], SECRET = [{}]", args.api_key, args.api_secret);
    let client = Client::builder()
        .build()
        .expect("Unable to build HTTP client.");
    make_request(
        client,
        "https://api.etsy.com/v3/application/openapi-ping",
        &args.api_key,
    )
}

fn make_request(client: Client, uri: &str, api_key: &str) {
    let req = client
        .get(uri)
        .header("x-api-key", api_key)
        .build()
        .expect("Unable to build the HTTP request.");
    let resp = client
        .execute(req)
        .expect(&format!("{}:{}", "HTTP request failed", uri))
        .json::<HashMap<String, String>>();
    println!("{:#?}", resp);
}
