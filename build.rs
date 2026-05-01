use std::{env, fmt::Write, fs, path::PathBuf};

const QUOTES: &[u8] = include_bytes!("quotes.json");

#[derive(serde::Deserialize)]
struct Quote<'a> {
    author: &'a str,
    text: &'a str,
    source: &'a str,
}

fn main() {
    println!("cargo:rerun-if-changed=quotes.json");

    let quotes: Box<[Quote]> = serde_json::from_slice(QUOTES).unwrap();
    assert!(!quotes.is_empty(), "empty :(");

    let entries = quotes
        .into_iter()
        .fold(String::new(), |mut entries, quote| {
            writeln!(
                entries,
                "\tQuote {{ author: {:?}, text: {:?}, source: {:?} }},",
                quote.author, quote.text, quote.source
            )
            .unwrap();
            entries
        });
    let output = format!("&[{entries}]");
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = PathBuf::from(out_dir).join("quotes.rs");
    fs::write(path, output).unwrap();
}
