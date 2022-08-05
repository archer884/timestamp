use chrono::Local;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
struct Args {
    /// an optional message printed with a timestamp
    message: Option<String>,
}

fn main() {
    let args = Args::parse();
    let now = Local::now();
    let now = now.format("%d %b %Y | %R");

    match args.message {
        Some(message) => println!("{now} | {message}"),
        None => println!("{now}"),
    }
}
