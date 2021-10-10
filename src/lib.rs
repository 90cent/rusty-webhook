// Sending Messages = POST    Editing Messages = PATCH
pub mod discord {
    use std::{collections::HashMap,result::*,error::Error};
    use ureq::{self, Response};
    use serde_json::{self,Result,Serializer};
    use serde::{Serialize,Deserialize};


    #[derive(Serialize,Deserialize,Debug)]
    pub struct webhook {
        avatar_url: String,
        username: String,
        content: String,
    }
    
    pub trait DiscordWebHook {
        fn send_raw(&self,url: &str);
        fn send(&self,url: &str);
        fn edit(&self, url: &str);
    }

    impl DiscordWebHook for webhook {
        //Make your own JSON
        fn send_raw(&self,url: &str) {
            let res = ureq::post(url)
            .set("Content-Type", "application/json")
            .send_string(&self.content).unwrap();
        }

        //Uses the Struct contents
        fn send(&self,url: &str){
            let content = serde_json::to_string(&self).unwrap();
            let res = ureq::post(url)
            .set("Content-Type", "application/json")
            .send_string(&content.as_str()).unwrap();
        }

        fn edit(&self, url: &str) {
            //Edit Discord Message with PATCH. 
            todo!()
        }

        
    }

    pub fn create_webhook(avatar_url: &str,nickname: &str,content: &'static str) -> webhook {
        let whook = webhook {avatar_url: avatar_url.to_string(), username: nickname.to_string(), content: content.to_string() };
        return whook;
    }
}
