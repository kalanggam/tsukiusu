// Rust port of rbaron/dict.cc.py
// Author - Gil Locaylocay Caley (kalanggam)
use std::collections::HashMap;
use std::error::Error;

use reqwest;
use url;

// SUPPORTED_LANGS
// ---------------
// The following language codes are supported:
//   en : English
//   de : German
const SUPPORTED_LANGS: &[&'static str] = &["en", "de"];

// DictccRequest
// Struct for HTTP requests to retrieve words from dict.cc. URI takes following form:
// ----------------
// https:// [source_lang] [target_lang] .dict.cc/ ?s= [search_term]
//          2-char code   2-char code                 search parameter
struct DictLookup {
    source_lang: String,
    target_lang: String,
    search: String,
}

impl DictLookup {
    pub fn new(source_lang: String, target_lang: String, search: String) -> DictLookup {
        DictLookup { source_lang, target_lang, search }
    }

    fn build_url(&self) -> Result<url::Url, url::ParseError> {
        let url = "https://".to_owned() + &self.source_lang + &self.target_lang + "dict.cc";
        let mut params = HashMap::new();
        params.insert("s", self.search.as_str());
        url::Url::parse_with_params(&url, &params)
    }

    pub fn lookup(&self) -> Result<String, Box<dyn Error>> {
        let url = self.build_url();
        if let Err(e) = url { Err(e)? }
        let resp = reqwest::blocking::get(url.unwrap())?.text()?;
        Ok(resp)
    }
}

fn lookup(source_lang: String, target_lang: String, search_term: String) -> Result<String, &'static str> {
    // Check if language codes
    if !(SUPPORTED_LANGS.iter().any(|&s| (s == &source_lang))) {
        return Err("unsupported source language code");
    } else if !(SUPPORTED_LANGS.iter().any(|&s| (s == &target_lang))) {
        return Err("unsupported target language code");
    }

    let req = DictLookup::new(source_lang, target_lang, search_term);
    let resp = req.lookup();
    match resp {
        Ok(text) => Ok(text),
        Err(_) => Err("failed request")
    }
}