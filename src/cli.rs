use wikipedia_cli::schema::WikiResponse;
use color_eyre::eyre::Result as ColoredResult;

// TODO: Make this dynamic and configurable
const BASE_URL: &str = "https://simple.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&redirects=1&formatversion=2&exintro=1&explaintext=1";

fn main() -> ColoredResult<()> {
    color_eyre::install()?;

    let titles = vec!["Pet Door", "Anesthetic"]
        .into_iter()
        .map(|s| s.into())
        .collect();

    let queries = WikiResponse::get(BASE_URL, titles)?.pages();
    println!("{queries:#?}");

    Ok(())
}
