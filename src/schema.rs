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

use serde::Deserialize;
use reqwest::{blocking::get, Result as HTTPResult};

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
  pub fn get(titles: Vec<&str>) -> HTTPResult<Self> {
    let title_str = titles.join("%7C");
    let url =
      format!("https://simple.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&titles={title_str}&redirects=1&formatversion=2&exintro=1&explaintext=1");

    let response: WikiResponse = get(url)?.json()?;
    match response.completed {
      true => Ok(response),
      false => unreachable!("`completed` fails as an empty string")
    }
  }

  /// Grabs the pages as obtained by the WikiResponse
  pub fn pages(&self) -> Vec<Page> {
    self.query.pages.clone()
  }
}