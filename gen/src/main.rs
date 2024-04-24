use clap::Parser;
use regex::Regex;
use scraper::{Html, Selector};

#[derive(Parser)]
struct Args {
    #[arg(
        long,
        default_value = "https://doc.rust-lang.org/1.77.2/std/convert/trait.From.html"
    )]
    url: String,
    #[arg(long, default_value = "section.impl > .code-header")]
    selector: String,
    #[arg(long)]
    regex: Option<Regex>,
}

fn main() -> anyhow::Result<()> {
    let Args {
        url,
        selector,
        regex: contains,
    } = Args::parse();
    let html = ureq::get(&url).call()?.into_string()?;
    let html = Html::parse_document(&html);
    for selected in
        html.select(&Selector::parse(&selector).map_err(|it| anyhow::Error::msg(it.to_string()))?)
    {
        let text = selected.text().collect::<Vec<_>>().join("");
        match &contains {
            Some(it) if it.is_match(&text) => println!("{}", text),
            Some(_) => {}
            None => println!("{}", text),
        }
    }
    Ok(())
}
