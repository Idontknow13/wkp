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

use crate::errors::WikiError;
use reqwest::blocking::get; // TODO: Change this to async with either std::thread or tokio
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WikiResponse {
    query: Query,
}

#[derive(Deserialize)]
struct Query {
    pages: Vec<Page>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Page {
    #[serde(rename = "pageid")]
    pub page_id: Option<usize>,
    pub title: String,
    pub extract: Option<String>,
    pub missing: Option<bool>,
}

impl WikiResponse {
    /// Fetches a response from Wikimedia given a specific title.
    pub fn get(url: String, titles: Vec<Title>) -> Result<Self, WikiError<'static>> {
        let title_strs = titles.to_str_vec();
        let url = format!("{url}&titles={}", title_strs.join("%7C"));

        Ok(get(url)?.json::<WikiResponse>()?)
    }

    /// Grabs the pages as obtained by `WikiResponse`.
    pub fn pages(&self) -> Vec<Page> {
        self.query.pages.clone()
    }
}

/// A string wrapper which allows normalization for requests made with the WikiMedia API.
#[derive(Clone)]
pub struct Title {
    normalized: bool,
    title: String,
}

trait ToStrVec {
    /// Turns `self` into a vector of strings.
    fn to_str_vec(&self) -> Vec<String>;
}

impl From<&str> for Title {
    fn from(s: &str) -> Self {
        Self::new(s).normalize()
    }
}

impl From<String> for Title {
    fn from(s: String) -> Self {
        Self::new(s).normalize()
    }
}

impl From<Title> for String {
    fn from(t: Title) -> Self {
        t.title
    }
}

impl Title {
    /// Creates a new, non-Default instance of `Title`.
    fn new<S: ToString>(title: S) -> Self {
        Self {
            normalized: false,
            title: title.to_string(),
        }
    }

    /// Normalizes the content of a `Title` instance.
    fn normalize(&self) -> Self {
        Self {
            normalized: true,
            title: normalize(&self.title),
        }
    }
}

impl ToStrVec for Vec<Title> {
    fn to_str_vec(&self) -> Vec<String> {
        self.iter()
            .map(|t| {
                let title = if !t.normalized {
                    t.normalize()
                } else {
                    t.clone()
                };

                String::from(title)
            })
            .collect()
    }
}

impl Page {
    pub fn get_wiki_url(&self, root_uri: &str) -> String {
        format!("{root_uri}/wiki/{}", normalize(&self.title))
    }
}

fn normalize(s: &str) -> String {
    s.replace(' ', "_")
}
