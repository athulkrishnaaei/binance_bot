// use dotenv::dotenv;
// use std::env;

// use teloxide::prelu&&de::*;

// #[tokio::main]
// async fn main() {
//     pretty_env_logger::init();
//     log::info!("Starting throw dice bot...");

//     let bot = Bot::from_env().auto_send();

//     teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
//         bot.send_dice(message.chat.id).await?;
//         respond(())
//     })
//     .await;
// }


// use binance::api::*;
// use binance::market::*;

// fn main() {
//     let market: Market = Binance::new(None, None);

//     // Latest price for ALL symbols
//     // match market.get_all_prices() {
//     //     Ok(answer) => println!("{:#?}", answer),
//     //     Err(e) => println!("Error: {:#?}", e),
//     // }

//     match market.get_price("BTCUSDT") {
//         Ok(answer) => println!("{:#?}", answer),
//         Err(e) => println!("Error: {:#?}", e),
//     }
// }

extern crate tldextract;

use teloxide::{prelude::*, utils::command::BotCommands};


// use url::{Url, ParseError};
use std::error::Error;

use binance::api::*;
use binance::market::*;

use tldextract::{TldExtractor, TldOption};

fn option() -> TldOption {
    TldOption::default()
}

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Price(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description ="shorten a passed url")]
    Shortener{urls: String},
    
  
    
}

async fn answer(
    
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let market: Market = Binance::new(None, None);
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Price(crpytocurrency) => {
            let mut iter = crpytocurrency.split_whitespace();

            if let Some(first_crypto_symbol) = iter.next() {

                let second_crypto_symbol = if let Some(second_crypto_symbol) = iter.next() {
                    println!("There was a second_crypto_symbol.");
                    second_crypto_symbol
                } else {
                    println!("There was no second_crypto_symbol. Use default.");
                    "USDT"
                };

                let target = to_uppercase(
                    &format!("{}{}", &first_crypto_symbol, &second_crypto_symbol)
                );

                match market.get_price(target) {
                    Ok(symbol_price) => {
                        println!("{:#?}", &symbol_price);
                        bot.send_message(message.chat.id,format!("The price you want is {:#?}. ", &symbol_price.price)).await?
                    },
                    Err(e) => {
                        eprint!("{:#?}", e);

                        bot.send_message(message.chat.id,format!("Something went wrong. Did you use the correct cryptocurrency pair?")).await?
                    },
                }
            } else {
                bot.send_message(message.chat.id,format!("Cryptocurrency symbols were not specified. To start with, you can use /price ETH or /price ETH USDT.")).await?
            }
        
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
        Command::Shortener{urls} => {
        let data = shorteners(urls);  
        if data.len()>0  {
        bot.send_message(message.chat.id,format!("shortened link is {:#?}",data)).await?
        } else {
            bot.send_message(message.chat.id,format!("Invalid url")).await?
        }
        }
    };

    Ok(())
}





fn shorteners(urls: String)->  String {
    let ext = TldExtractor::new(option());
    
    let tld = ext.extract(&urls).unwrap();
   
    let mut subdomain = String::from(tld.subdomain.unwrap());
    let domain = tld.domain.unwrap();
    let suffix = tld.suffix.unwrap();
    subdomain.push_str(".");
    subdomain.push_str(&domain);
    subdomain.push_str(".");
    subdomain.push_str(&suffix);
    subdomain.to_string();
   
    return subdomain;
}
