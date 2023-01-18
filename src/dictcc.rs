/* DICTCC.RS
 * Author - Gil Locaylocay Caley (kalanggam)
 * 
 * dictcc.rs is a Rust port of rbaron/dict.cc.py which uses reqwest (a wrapper of hyper), 
 */
// external imports
use std::collections::HashMap;
use reqwest;
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
// https:// [source_lang] [target_lang] .dict.cc/ ?s= [search_term]
//          2-char code   2-char code                 search parameter
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
        let url = "https://".to_owned() + &source + &target + "dict.cc";
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

// lookup
// This function is what users can use
pub fn lookup(source_lang: String, target_lang: String, search_term: String) -> Result<String, DictError> {
    // Check if language codes match supported languages
    if !(SUPPORTED_LANGS.iter().any(|&s| (s == &source_lang))) {
        return Err(DictError::UnsupportedLang(source_lang));
    } else if !(SUPPORTED_LANGS.iter().any(|&s| (s == &target_lang))) {
        return Err(DictError::UnsupportedLang(target_lang));
    }

    let req = DictLookup::build(source_lang, target_lang, search_term)?;
    let resp = req.get_response()?;
    Ok(resp)
}