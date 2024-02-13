/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */

//! # Paywallremover
//! Library to add the removepaywall prefix to the url. If the url is a medium url, freedium will be prefixed.\
//! This library rely on a `domains.json` file. For more information check the [README.md](https://github.com/matteof04/paywallremover/master/README.md) file on GitHub.

use log::info;
use special::{is_valid_special_url, Domains};
use thiserror::Error;
use url::Url;

/// Functions for special domains
pub mod special;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("The provided text is not a valid URL")]
    NotAnUrl,
    #[error("The provided text is not a valide base")]
    NotABase,
}

/// Check if the url is special or not.\
/// To do this check it uses the provided [Domains].\
/// If the `remote_check` variable is set, the function fetch the site content and scrape to find the special tag.
pub async fn process_url(
    domains: &Domains,
    mut url: Url,
    remote_check: bool,
) -> Result<String, ProcessError> {
    if url.cannot_be_a_base() {
        return Err(ProcessError::NotABase);
    }
    let freedium = "https://freedium.cfd/";
    let remove_paywall = "https://removepaywall.com/";
    let mut response_url = String::new();
    if domains.is_special(&url) {
        response_url.push_str(freedium);
        url.set_query(None);
        response_url.push_str(url.as_str())
    } else if remote_check {
        if domains.is_not_special(&url) {
            response_url.push_str(remove_paywall);
            response_url.push_str(url.as_str())
        } else if is_valid_special_url(&url).await {
            info!("Found new special URL: {url}");
            info!("Add this to domains.json for faster response");
            response_url.push_str(freedium);
            url.set_query(None);
            response_url.push_str(url.as_str());
        } else {
            response_url.push_str(remove_paywall);
            response_url.push_str(url.as_str());
        }
    } else {
        response_url.push_str(remove_paywall);
        response_url.push_str(url.as_str());
    }
    Ok(response_url)
}
