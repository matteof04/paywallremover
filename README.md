# Paywallremover

 Url updater bot for Telegram

## Setup

Before program execution set the BOT_TOKEN environment variable with the token provided by BotFather.\
By default, the LOG_LEVEL environment variable is set to INFO.\
If you want extremly verbose and detailed output, you can set it to TRACE or DEBUG.\
If you want less output you can set it to WARN, or for even less output, to ERROR, although this should be avoided.

## Usage

Send a message with an URL of a site you want to remove the paywall and the bot will reply to you with `removepaywall.com/yoururl`.
If the site is a Medium site, this will reply with the corresponding Freedium link and it will automatically remove Medium tracking GET arguments.
Due to the fact that they are treated differently, the medium urls are considered "special".
To avoid that the program fetch and scrape the site every time to check if it is special or not, the `domains.json` file is used.
To build the json you can check [here](https://github.com/Freedium-cfd/core/blob/dde0acfc62e5afee13cfa5308eb2a45292e9d886/medium_parser/utils.py) and create a `domains.json` file where the program is executed, using the following template:

```json
{
    "special_domains": [
        "medium.com"
    ],
    "not_special_domains": [
        "github.com",
        "reddit.com"
    ]
}

```

If you want to use a different name or path for the domains file, set the DOMAIN_FILE environment variable to the desired path.

A docker compose example file is available [here](https://raw.githubusercontent.com/matteof04/paywallremover/refs/heads/main/compose.yaml).

## Note

This software nor its creator are affiliated with Medium, Freedium or Paywallremover. For copyright complaints about any sort, contact them.\
If you use this software, as stated in the `LICENSE` file, you accept that I am not liable for any use of the software that violates the terms and conditions of any service.
