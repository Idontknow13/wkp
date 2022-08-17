/*!
This module contains the custom `WikiError` type.
*/

use reqwest::Error as HTTPError;

#[derive(Debug)]
pub struct WikiError<'a> {
    err: HTTPError,
    cause: Option<&'a str>,
    args: Option<Vec<String>>,
}

fn verify_cause(err: &HTTPError) -> &'static str {
    match &err {
        e if e.is_decode() => "title",
        e if e.is_redirect() => "redirect",
        e if e.is_request() || e.is_timeout() => "request",
        e if e.is_body() || e.is_status() => "response",
        e if e.is_builder() || e.is_connect() => unimplemented!("not handled externally"),
        _ => unreachable!("all error kinds in `reqwest` is handled"),
    }
}

impl<'a> WikiError<'a> {
    /// Creates a new `WikiError instance.`
    pub fn new(err: HTTPError, cause: &'a str, args: Vec<String>) -> Self {
        WikiError {
            err,
            cause: Some(cause),
            args: Some(args),
        }
    }
}

impl From<HTTPError> for WikiError<'_> {
    fn from(err: HTTPError) -> Self {
        Self {
            cause: Some(verify_cause(&err)), // allows borrow before move
            err,
            args: Some(vec![]),
        }
    }
}

impl std::fmt::Display for WikiError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
        f,
        "One of the {cause}s didn't yield a proper response.\nProvided args: {args:?}\nError: {err}",
        cause = self.cause.unwrap_or("request"),
        args = self.args,
        err = self.err
        )
    }
}

impl std::error::Error for WikiError<'_> {}
