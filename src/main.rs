static QUOTES: &[Quote] = include!(concat!(env!("OUT_DIR"), "/quotes.rs"));

struct Quote {
    author: &'static str,
    text: &'static str,
    source: &'static str,
}

fn main() {
    let quote = &QUOTES[fastrand::usize(..QUOTES.len())];
    println!("{}\n~ {} ({})", quote.text, quote.author, quote.source);
}
