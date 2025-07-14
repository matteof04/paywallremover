/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */

use std::{env, process::exit, sync::Arc};

use log::{error, info, trace, warn};
use paywallremover::{process_url, special::Domains, ProcessError};
use regex::Regex;
use teloxide::{
    prelude::*,
    types::{MessageEntityKind, ReplyParameters},
};
use url::Url;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("LOG_LEVEL")
        .init();
    let bot_token = match env::var("BOT_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("BOT_TOKEN not set!");
            exit(1)
        }
    };
    let remote_check = match env::var("REMOTE_CHECK") {
        Ok(mrc) => match mrc.parse::<bool>() {
            Ok(b) => b,
            Err(_) => {
                warn!("REMOTE_CHECK incorrectly formatted, defaulting to false.");
                false
            }
        },
        Err(_) => {
            info!("REMOTE_CHECK not set, defaulting to false.");
            false
        }
    };
    let remote_check = Arc::new(remote_check);
    let domain_path = match env::var("DOMAIN_FILE") {
        Ok(path) => path,
        Err(_) => {
            info!("DOMAIN_FILE not set, defaulting to domains.json");
            String::from("domains.json")
        }
    };
    let domains = Arc::new(Domains::load(domain_path));
    let bot = Bot::new(bot_token);
    let handler = Update::filter_message().endpoint(
        |bot: Bot, remote_check: Arc<bool>, domains: Arc<Domains>, msg: Message| async move {
            if let Some(user) = &msg.from {
                trace!("New message from user with ID: {:?}", user.id)
            }
            match extract_url(&msg) {
                Some(url_result) => match url_result {
                    Ok(url) => {
                        let response = match process_url(&domains, url, *remote_check).await {
                            Ok(url) => url,
                            Err(e) => format!("{e}"),
                        };
                        bot.send_message(msg.chat.id, response)
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("{e}"))
                            .reply_parameters(ReplyParameters::new(msg.id))
                            .await?;
                    }
                },
                None => {
                    bot.send_message(msg.chat.id, format!("{}", ProcessError::NotAnUrl))
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await?;
                }
            }
            respond(())
        },
    );
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![remote_check, domains])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn extract_url(msg: &Message) -> Option<Result<Url, ProcessError>> {
    let mut urls: Vec<Result<Url, ProcessError>> = vec![];
    if let Some(entities) = msg.parse_entities() {
        let mut parsed_text_links: Vec<Result<Url, ProcessError>> = entities
            .into_iter()
            .filter_map(|e| {
                if let MessageEntityKind::TextLink { url } = e.kind() {
                    Some(Ok(url.to_owned()))
                } else {
                    None
                }
            })
            .collect();
        urls.append(&mut parsed_text_links);
    }
    if let Some(entities) = msg.parse_caption_entities() {
        let mut parsed_caption_links: Vec<Result<Url, ProcessError>> = entities
            .into_iter()
            .filter_map(|e| {
                if let MessageEntityKind::TextLink { url } = e.kind() {
                    Some(Ok(url.to_owned()))
                } else {
                    None
                }
            })
            .collect();
        urls.append(&mut parsed_caption_links);
    }
    let msg_text = msg.text().unwrap_or("");
    let mut parsed_msg_text = parse_url(msg_text);
    urls.append(&mut parsed_msg_text);
    let caption_text = msg.caption().unwrap_or("");
    let mut parsed_caption_text = parse_url(caption_text);
    urls.append(&mut parsed_caption_text);
    urls.into_iter().next()
}

fn parse_url(text: &str) -> Vec<Result<Url, ProcessError>> {
    Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)")
        .unwrap()
        .find_iter(text)
        .map(|m| m.as_str())
        .map(|s| Url::parse(s).map_err(|_| ProcessError::NotAnUrl))
        .collect()
}
