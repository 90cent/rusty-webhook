// Sending Messages = POST    Editing Messages = PATCH
pub mod discord {
    use surf::{self,http::mime};
    use std::{collections::HashMap,result::*,error::Error};
    use tokio;

    use serde_json::{self,Result,Serializer, Value};
    use serde::{Serialize,Deserialize,self};


    #[derive(Serialize)]
    pub struct Webhook {
        avatar_url: String,
        username: String,
        content: String,
    }


    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WebhookHandle {
        pub id: String,
        #[serde(rename = "type")]
        pub type_field: i64,
        pub content: String,
        #[serde(rename = "channel_id")]
        pub channel_id: String,
        pub author: Author,
        pub attachments: Vec<Value>,
        pub embeds: Vec<Value>,
        pub mentions: Vec<Value>,
        #[serde(rename = "mention_roles")]
        pub mention_roles: Vec<Value>,
        pub pinned: bool,
        #[serde(rename = "mention_everyone")]
        pub mention_everyone: bool,
        pub tts: bool,
        pub timestamp: String,
        #[serde(rename = "edited_timestamp")]
        pub edited_timestamp: Value,
        pub flags: i64,
        pub components: Vec<Value>,
        #[serde(rename = "webhook_id")]
        pub webhook_id: String,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Author {
        pub bot: bool,
        pub id: String,
        pub username: String,
        pub avatar: String,
        pub discriminator: String,
    }


    pub trait WebHook {
        fn send(&self,url: &str);
    }

    impl WebHook for Webhook {
        //Uses the Struct contents
        fn send(&self,url: &str){
            let content = serde_json::to_string(&self).unwrap();
            let _url = format!("{}?wait=true",url);
            drop(url);

            let rt = tokio::runtime::Builder::new_current_thread()
                .max_blocking_threads(2)
                .on_thread_start(|| { println!("Tokio Thread Started"); })
                .build().expect("Couldnt create tokio runtime");

            
            let response = rt.block_on(async move {
                let post = surf::post(_url)
                    .body_string(content)
                    .content_type(mime::JSON)
                    .send().await;

                post.expect("Error").body_string().await.unwrap()
            });
            
            
            
        }
    }

    
    pub fn create_webhook(avatar_url: &str,nickname: &str,content: String) -> Webhook {
        Webhook {avatar_url: avatar_url.to_string(), username: nickname.to_string(), content: content }
    }
}



pub mod Components {
    
}