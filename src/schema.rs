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
use reqwest::{blocking::get, Error as ReqwestErr};
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
    page_id: usize,
    title: String,
    extract: Option<String>,
    missing: Option<bool>,
}

impl WikiResponse {
    /// Fetches a response from Wikimedia given a specific title.
    pub fn get(url: &str, titles: Vec<Title>) -> Result<Self, WikiError<'static>> {
        let title_str = titles.chain("%7C");
        let url = format!("{url}&titles={}", title_str);

        match get(url)?.json::<WikiResponse>() {
            Ok(resp) => Ok(resp),
            Err(err) => Err(WikiError::RequestError {
                err,
                ident: "titles",
                args: titles.to_str_vec(),
            }),
        }
    }

    /// Grabs the pages as obtained by `WikiResponse`.
    pub fn pages(&self) -> Vec<Page> {
        self.query.pages.clone()
    }
}

/// A string wrapper which allows normalization and title chaining
/// for requests made with the WikiMedia API.
#[derive(Clone)]
pub struct Title {
    normalized: bool,
    title: String,
}

trait ToStrVec {
    /// Turns `self` into a vector of strings.
    fn to_str_vec(&self) -> Vec<String>;

    /// Joins `self` into a single string.
    fn chain(&self, _: &str) -> String;
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
    /// Creates a new, non-Default instance of `Title`.
    fn new(title: &str) -> Self {
        Self {
            normalized: false,
            title: title.to_string(),
        }
    }

    /// Normalizes the content of a `Title` instance.
    fn normalize(&self) -> Self {
        Self {
            normalized: true,
            title: self.title.replace(" ", "_"),
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

    fn chain(&self, sep: &str) -> String {
        self.to_str_vec().join(sep)
    }
}