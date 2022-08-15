use color_eyre::eyre::Result as ColoredResult;
use colored::Colorize;
use wikipedia_cli::{
    schema::{Page, WikiResponse},
    urlbuilder::{ToURL, WikiURL},
};

// TODO: Add the CLI

enum PrintOptions {
    FirstParagraphOnly,
    All,
}

fn main() -> ColoredResult<()> {
    color_eyre::install()?;

    let url = WikiURL::default();
    let titles = vec!["Sex Pistols"].into_iter().map(|s| s.into()).collect();

    let queries = WikiResponse::get(url.to_url(), titles)?.pages();
    print_pages(queries, url.get_root_uri(), None);

    Ok(())
}

fn print_pages(pages: Vec<Page>, uri: String, options: Option<PrintOptions>) {
    let options = options.unwrap_or_default();

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

        let extract = page
            .extract
            .as_ref()
            .expect("the extract should exist if the page exists");
        println!(
            "  ~ {extract}\n",
            extract = match &options {
                &PrintOptions::All => extract,
                &PrintOptions::FirstParagraphOnly => extract
                    .split("\n")
                    .next()
                    .expect("first paragraph is guaranteed to exist"),
            }
        );

        let page_url = page.get_wiki_url(&uri);
        println!(
            "{read_more}",
            read_more = format!("Read more at {page_url}...").truecolor(128, 128, 128)
        )
    }
}

impl Default for PrintOptions {
    fn default() -> Self {
        Self::FirstParagraphOnly
    }
}
