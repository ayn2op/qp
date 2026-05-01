use std::process;

use clap::Parser;
use rand::seq::IteratorRandom;
use serde::Deserialize;

const QUOTES: &[u8] = include_bytes!("../quotes.json");

#[derive(Deserialize)]
struct Quote<'a> {
    author: &'a str,
    text: &'a str,
    source: &'a str,
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_delimiter = ',')]
    author: Vec<String>,
}

fn main() -> serde_json::Result<()> {
    let args = Args::parse();

    let quotes: Box<[Quote]> = serde_json::from_slice(QUOTES)?;
    if quotes.is_empty() {
        eprintln!("quotes must not be empty");
        process::exit(1);
    }

    let Some(quote) = quotes
        .into_iter()
        .filter(|quote| {
            args.author.is_empty() || args.author.iter().any(|author| author == quote.author)
        })
        .choose(&mut rand::rng())
    else {
        eprintln!("no quotes matched the author filter");
        process::exit(1);
    };
    println!("{}\n~ {} ({})", quote.text, quote.author, quote.source);
    Ok(())
}
