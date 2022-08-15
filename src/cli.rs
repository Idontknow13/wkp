use color_eyre::eyre::Result as ColoredResult;
use colored::Colorize;
use wikipedia_cli::schema::{Page, WikiResponse};

// TODO: Make this configurable
const BASE_URL: &str = "https://simple.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&redirects=1&formatversion=2&exintro=1&explaintext=1";

fn main() -> ColoredResult<()> {
    color_eyre::install()?;

    let titles = vec!["Sex Pistols"].into_iter().map(|s| s.into()).collect();

    let queries = WikiResponse::get(BASE_URL, titles)?.pages();
    print_pages(queries);

    Ok(())
}

fn print_pages(pages: Vec<Page>) {
    for page in pages {
        if page.missing.is_some() {
            println!(
                "{title_msg}\n{help}",
                title_msg = format!(r#""{}" does not exist."#, page.title).bright_red(),
                help = "On compound item names, it is recommended to have the second word in lowercase rather than uppercase.".bright_red()
            );
            continue;
        }

        //* Title: Navy Blue Bg + Yellow Fg
        //* Page ID: Gray
        //* Content: No color
        println!(
            "[ {title} ]  {page_id}",
            title = page.title.bold(),
            page_id = format!("(Page ID: {})", page.page_id.unwrap()).truecolor(128, 128, 128)
        );
        let extract = page.extract.unwrap_or_default();
        let extract = extract.split("\n").next().unwrap_or_default();
        println!("  ~ {extract}\n")

        // TODO: Add a `Read more...` at the very end
    }
}
