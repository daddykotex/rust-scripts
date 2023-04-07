use clap::Parser;
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
    make_request("https://httpbin.org/ip")
}

fn make_request(uri: &str) {
    let resp = reqwest::blocking::get(uri)
        .expect(&format!("{}:{}", "HTTP request failed", uri))
        .json::<HashMap<String, String>>();
    println!("{:#?}", resp);
}
