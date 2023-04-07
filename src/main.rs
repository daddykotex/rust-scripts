use clap::Parser;

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
}
