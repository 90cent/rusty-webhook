// Sending Messages = POST    Editing Messages = PATCH
pub mod discord {
    use std::{collections::HashMap,result::*,error::Error,env};
    use ureq::{self, Response};
    use serde_json::{self,Result,Serializer};
    use serde::{Serialize,Deserialize};
    

    #[derive(Serialize)]
    pub struct Webhook {
        avatar_url: String,
        username: String,
        content: String,
    }

    pub struct WebhookHandle {
        webhook: Webhook,
        message_id: u32
    }
    
    pub trait WebHook {
        fn send_raw(&self,url: &str);
        fn send(&self,url: &str);
    }

    impl WebHook for Webhook {
        //Make your own JSON
        fn send_raw(&self,url: &str) {
            let res = ureq::post(url)
            .set("Content-Type", "application/json")
            .send_string(&self.content).unwrap();
        }

        //Uses the Struct contents
        fn send(&self,url: &str){
            let content = serde_json::to_string(&self).unwrap();
            let res = surf::post(url)
                .body_string(content)
                .recv_string();
            
            match res {
            
            }
        }
    }

    
    pub fn create_webhook(avatar_url: &str,nickname: &str,content: String) -> Webhook {
        Webhook {avatar_url: avatar_url.to_string(), username: nickname.to_string(), content: content }
    }
}


pub mod Components {
    struct 
}