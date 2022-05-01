use std::{env,thread};
use rusty_webhook::discord::{Webhook,WebHook, self};


pub fn main() {
    let arguments: Vec<String> = env::args().collect();

    println!("{:#?}",arguments);

    if arguments.len() < 4 {
        println!("SYNTAX: URL NICKNAME AVATAR-URL CONTENT aka message")
    }

    let url = &arguments[1];
    let nickname = &arguments[2];
    let avatar = &arguments[3];
    let content = arguments[4].clone();


    let webhook = discord::create_webhook(avatar.as_str(), nickname.as_str(),content);
    webhook.send(url);
}

