use reqwest::Error as HTTPError;

#[derive(Debug)]
pub enum WikiError<'a> {
    RequestError {
        err: HTTPError,
        ident: &'a str,
        args: Vec<String>,
    },
}

impl From<HTTPError> for WikiError<'_> {
    fn from(err: HTTPError) -> Self {
        Self::RequestError {
            err,
            ident: "",
            args: vec![],
        }
    }
}

impl std::fmt::Display for WikiError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestError { err, ident, args } => write!(
                f,
                "One of the {ident} didn't yield a proper response.
                Provided args: {args:?}
                Error: {err:?}
                ",
            ),
        }
    }
}

impl std::error::Error for WikiError<'_> {}
