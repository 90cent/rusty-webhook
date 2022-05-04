use std::{env,thread};
use rusty_webhook::discord::{Webhook,WebHook, self};


pub fn main() {
    let arguments: Vec<String> = env::args().collect();

    println!("{:#?}",arguments);

    if arguments.len() < 5 {
        println!("SYNTAX: MODE(send,edit) URL NICKNAME AVATAR-URL CONTENT aka message (if edit pass message id as last argument)");
    }

    let mode = &*arguments[1];
    let url = &arguments[2];
    let nickname = &arguments[3];
    let avatar = &arguments[4];
    let content = arguments[5].clone();

    let webhook = discord::create_webhook(avatar.as_str(), nickname.as_str(),content);
    
    
}

