/*!
This module contains the struct and methods which allow request URL building.
*/

pub struct WikiURL {
    subdomain: WikiSubdomain,
    queries: QueryParams,
}

pub enum WikiSubdomain {
    SimpleWikipedia,
    Wikipedia
}

pub struct QueryParams {
    format: Format,
    formatversion: FormatVersion,
    redirects: bool,
    prop: Prop,
}

pub enum Format {
    JSON,
    PHP,
    XML,
    Debug,
    None,
}

pub enum Prop {
    Extracts { intro_only: bool, plaintext: bool },
    Revisions,
}

pub enum FormatVersion {
    BackwardsCompatible,
    Modern,
}

pub trait ToURL {
    fn to_url(&self) -> String;
}

impl WikiURL {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_subdomain(mut self, subdomain: WikiSubdomain) -> Self {
        self.subdomain = subdomain;
        self
    }

    pub fn with_props(mut self, q: QueryParams) -> Self {
        self.queries = q;
        self
    }

    pub fn get_root_uri(&self) -> String {
        format!("https://{}", self.subdomain.to_string())
    }
}

impl Default for WikiURL {
    fn default() -> Self {
        Self {
            subdomain: WikiSubdomain::SimpleWikipedia,
            queries: QueryParams {
                format: Format::JSON,
                formatversion: FormatVersion::Modern,
                redirects: true,
                prop: Prop::Extracts {
                    intro_only: true,
                    plaintext: true,
                },
            },
        }
    }
}

impl ToURL for WikiURL {
    fn to_url(&self) -> String {
        format!(
            "https://{domain}/w/api.php?{queries}",
            domain = self.subdomain.to_string(),
            queries = self.queries.to_url()
        )
    }
}

impl ToURL for QueryParams {
    fn to_url(&self) -> String {
        let mut queryparams = vec![
            "action=query".to_string(),
            self.format.to_url(), // format={}
            self.formatversion.to_url(), // formatversion={}
            self.prop.to_url(), // prop={}&...
        ];

        if self.redirects {
            queryparams.push("redirects=1".to_string());
        }

        queryparams.join("&")
    }
}

impl ToURL for Format {
    fn to_url(&self) -> String {
        format!(
            "format={}",
            match self {
                Self::JSON => "json",
                Self::PHP => "php",
                Self::XML => "xml",
                Self::Debug => "rawfm",
                Self::None => "none",
            }
        )
    }
}

impl ToURL for FormatVersion {
    fn to_url(&self) -> String {
        let formatversion: u8 = match self {
            Self::BackwardsCompatible => 1,
            Self::Modern => 2,
        };

        format!("formatversion={formatversion}")
    }
}

impl ToURL for Prop {
    fn to_url(&self) -> String {
        let props = match self {
            Self::Extracts {
                intro_only,
                plaintext,
            } => {
                let mut propvec = vec!["prop=extracts"];

                if *intro_only {
                    propvec.push("exintro=1");
                }
                if *plaintext {
                    propvec.push("explaintext=1");
                }

                propvec.clone()
            }
            Self::Revisions => vec!["prop=revisions"],
        };

        props.join("&").to_string()
    }
}

impl ToString for WikiSubdomain {
    fn to_string(&self) -> String {
        match self {
            WikiSubdomain::SimpleWikipedia => "simple.wikipedia.org",
            WikiSubdomain::Wikipedia => "en.wikipedia.org",
        }.to_string()
    }
}

#[cfg(test)]
mod tests;
