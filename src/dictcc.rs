/* DICTCC.RS
 * Author - Gil Locaylocay Caley (kalanggam)
 * 
 * TODO: Parse HTML DOM into a returnable format
 */
// external imports
use std::collections::HashMap;
use reqwest;
use scraper::{Html, Selector, ElementRef};
use url;

// internal imports
use error::DictError;

// modules
pub mod error;

// SUPPORTED_LANGS
// ---------------
// The following language codes are supported:
//   en : English
//   de : German
const SUPPORTED_LANGS: &[&'static str] = &["en", "de"];

// DictLookup
// Struct for HTTP requests to retrieve words from dict.cc. URL takes following form:
// ----------------
// https:// [source_lang] - [target_lang] .dict.cc/ ?s= [search_term]
//          2-char code     2-char code                 search parameter
// The url is parsed immediately when DictLookup is built using `build`.
struct DictLookup {
    url: url::Url,
}

impl DictLookup {
    // execute
    fn execute(&self) -> Result<reqwest::blocking::Response, DictError> {
        let url = self.url.clone();
        match reqwest::blocking::get(url) {
            Ok(resp) => Ok(resp),
            Err(e) => Err(DictError::HttpError(e)),
        }
    }

    // get_response
    fn get_response(&self) -> Result<String, DictError> {
        let resp = self.execute()?;
        match resp.text() {
            Ok(s) => Ok(s),
            Err(e) => Err(DictError::ResponseError(e)),
        }
    }

    // get_dict_url
    fn get_dict_url(source: String, target: String, search: String) -> Result<url::Url, DictError> {
        let url = "https://".to_owned() + &source + "-" + &target + ".dict.cc";
        let mut params = HashMap::new();
        params.insert("s", search.as_str());
        match url::Url::parse_with_params(&url, &params) {
            Ok(url) => Ok(url),
            Err(e) => Err(DictError::ParseError(e)),
        }
    }

    // build
    fn build(source_lang: String, target_lang: String, search: String) -> Result<DictLookup, DictError> {
        let url = Self::get_dict_url(source_lang, target_lang, search)?;
        Ok(DictLookup { url })
    }
}

fn get_terms(doc: String) -> Option<HashMap<String, String>> {
    let document = Html::parse_document(&doc);
    let mut terms = HashMap::new();
    let selector = Selector::parse(".td7nl").unwrap();
    let cells = document.select(&selector);

    for item in cells {
        if item.value().name().to_string() == "div" { continue } // skip langbar
        print!("text parsing: ");
        for child in item.children() {
            if child.value().is_text() {
                let text = child.value().as_text().unwrap();
                if !text.is_empty() {
                    print!("{}", text.to_string());
                }
            }
            if child.value().is_element() {
                let elem = child.value().as_element().unwrap();
                match elem.name() {
                    "dfn" => (),
                    "div" => (),
                    "sup" => print!(" ({})", ElementRef::wrap(child).unwrap().text().collect::<Vec<_>>().join("").to_string()),
                    _ => print!("{}", ElementRef::wrap(child).unwrap().text().collect::<Vec<_>>().join("").to_string()),
                }
            }
        }
        println!("");
    }

    Some(terms)
}

// search
// This function is what users can use
pub fn search(source_lang: String, target_lang: String, search_term: String) -> Result<HashMap <String, String>, DictError> {
    // Check if language codes match supported languages
    if !(SUPPORTED_LANGS.iter().any(|&s| (s == &source_lang))) {
        return Err(DictError::UnsupportedLang(source_lang));
    } else if !(SUPPORTED_LANGS.iter().any(|&s| (s == &target_lang))) {
        return Err(DictError::UnsupportedLang(target_lang));
    } else {
        let lookup = DictLookup::build(source_lang, target_lang, search_term)?;
        let resp = get_terms(lookup.get_response()?).unwrap();
        Ok(resp)
    }
}