/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */

use log::warn;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tap::Tap;
use thiserror::Error;
use url::Url;

/// The list of the known special domains and the known not special domains
#[derive(Serialize, Deserialize, Default)]
pub struct Domains {
    special_domains: Vec<String>,
    not_special_domains: Vec<String>,
}

impl Domains {
    /// Load from the file at the provided path the domains lists.
    pub fn load(path: String) -> Self {
        let data = std::fs::read_to_string(path)
            .tap(|var| {
                if var.is_err() {
                    warn!("Domains file not found, domain list defaulting to empty")
                }
            })
            .unwrap_or_default();
        serde_json::from_str(&data)
            .tap(|var| {
                if var.is_err() {
                    warn!("Domains file incorrectly formatted, domain list defaulting to empty")
                }
            })
            .unwrap_or_default()
    }
    /// Check if the given url is known special domain
    pub fn is_special(&self, url: &Url) -> bool {
        let domain = url.domain();
        match domain {
            Some(domain) => {
                let domain = String::from(domain);
                self.special_domains.contains(&domain)
            }
            None => false,
        }
    }
    /// Check if the given url is known not special domain
    pub fn is_not_special(&self, url: &Url) -> bool {
        let domain = url.domain();
        match domain {
            Some(domain) => {
                let domain = String::from(domain);
                self.not_special_domains.contains(&domain)
            }
            None => false,
        }
    }
}
/// Fetch the content of the given url to check if it is special or not
pub async fn is_valid_special_url(url: &Url) -> bool {
    _is_valid_special_url(url).await.is_ok()
}

async fn _is_valid_special_url(url: &Url) -> Result<(), Box<dyn Error>> {
    let url = url.clone();
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let document = scraper::Html::parse_document(&text);
    let selector =
        scraper::Selector::parse("head > meta[property=\"og:site_name\"][content=\"Medium\"]")?;
    match document.select(&selector).next() {
        Some(_) => Ok(()),
        None => Err(Box::new(SpecialSearchError::NotSpecial)),
    }
}

#[derive(Error, Debug)]
pub enum SpecialSearchError {
    #[error("The provided text is not a valide base")]
    NotSpecial,
}
