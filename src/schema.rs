/*!
This module contains the schema used to parse the response
sent by the Wikimedia API using these options:

- `action=query`
- `format=json`
- `prop=extracts`
- `redirects=1`
- `formatversion=2`
- `exintro=1`
- `explaintext=1`
*/

use reqwest::{blocking::get, Result as HTTPResult};
use serde::Deserialize;

const BASE_URL: &str = "https://simple.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&redirects=1&formatversion=2&exintro=1&explaintext=1";

#[derive(Deserialize)]
pub struct WikiResponse {
    #[serde(rename = "batchcomplete")]
    completed: bool,
    query: Query,
}

#[derive(Deserialize)]
struct Query {
    pages: Vec<Page>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Page {
    #[serde(rename = "pageid")]
    page_id: usize,
    title: String,
    extract: Option<String>,
    missing: Option<bool>,
}

impl WikiResponse {
    /// Fetches a response from Wikimedia given a specific title
    pub fn get(titles: Vec<Title>) -> HTTPResult<Self> {
        let title_strs = Title::to_str_vec(titles);
        let url = format!("{BASE_URL}&titles={}", title_strs.join("%7C"));

        let response: WikiResponse = get(url)?.json()?;
        match response.completed {
            true => Ok(response),
            false => unreachable!("`completed` fails as an empty string"),
        }
    }

    /// Grabs the pages as obtained by the WikiResponse
    pub fn pages(&self) -> Vec<Page> {
        self.query.pages.clone()
    }
}

pub struct Title {
    normalized: bool,
    title: String,
}

impl From<&str> for Title {
    fn from(s: &str) -> Self {
        Title::new(s).normalize()
    }
}

impl From<Title> for String {
    fn from(t: Title) -> Self {
        t.title
    }
}

impl Title {
    fn new(title: &str) -> Self {
        Self {
            normalized: false,
            title: title.to_string(),
        }
    }

    fn normalize(self) -> Self {
        Self {
            normalized: true,
            title: self.title.replace(" ", "_"),
        }
    }

    fn to_str_vec(titles: Vec<Self>) -> Vec<String> {
        titles
            .into_iter()
            .map(|t| {
                let title = if !t.normalized { t.normalize() } else { t };

                String::from(title)
            })
            .collect()
    }
}
