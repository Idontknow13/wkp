/*!
This module is for testing the `urlbuilder` module located at the src/ directory.
*/

use super::*;
use test_case::test_case;

// ---- `ToURL` tests ---- //

#[test]
fn wikiurl_to_url_works() {
    let wikiurl = WikiURL::default()
        .with_subdomain("a")
        .with_props(QueryParams {
            format: Format::None,
            formatversion: FormatVersion::BackwardsCompatible,
            redirects: false,
            prop: Prop::Revisions,
        });

    assert_eq!(
        "https://a/w/api.php?action=query&format=none&formatversion=1&prop=revisions",
        wikiurl.to_url()
    )
}

#[test]
fn query_to_url_works() {
    let sample_query = QueryParams {
        format: Format::JSON,
        formatversion: FormatVersion::Modern,
        redirects: true,
        prop: Prop::Extracts {
            intro_only: true,
            plaintext: true,
        },
    }.to_url();

    assert_eq!(
        "action=query&format=json&formatversion=2&prop=extracts&exintro=1&explaintext=1&redirects=1",
        sample_query
    )
}

#[test_case(Format::JSON  => "format=json" ; "when format is JSON")]
#[test_case(Format::PHP   => "format=php"  ; "when format is php")]
#[test_case(Format::XML   => "format=xml"  ; "when format is xml")]
#[test_case(Format::Debug => "format=rawfm"; "when format is rawfm")]
#[test_case(Format::None  => "format=none" ; "when format is none")]
fn format_to_url_works(fmt: Format) -> String {
    fmt.to_url()
}

#[test_case(FormatVersion::BackwardsCompatible => "formatversion=1" ; "when formatversion is legacy")]
#[test_case(FormatVersion::Modern              => "formatversion=2" ; "when formatversion is modern")]
fn formatver_to_url_works(fmtver: FormatVersion) -> String {
    fmtver.to_url()
}

#[test_case(Prop::Extracts{ intro_only: false, plaintext: false } => "prop=extracts"                         ; "when prop is extracts")]
#[test_case(Prop::Extracts{ intro_only: true, plaintext: false }  => "prop=extracts&exintro=1"               ; "when intro_only is enabled")]
#[test_case(Prop::Extracts{ intro_only: false, plaintext: true }  => "prop=extracts&explaintext=1"           ; "when plaintext is enabled")]
#[test_case(Prop::Extracts{ intro_only: true, plaintext: true }   => "prop=extracts&exintro=1&explaintext=1" ; "when extracts is fully enabled")]
#[test_case(Prop::Revisions => "prop=revisions" ; "when prop is revisions")]
fn props_to_url_works(props: Prop) -> String {
    props.to_url()
}
