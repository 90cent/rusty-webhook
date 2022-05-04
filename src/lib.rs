// Sending Messages = POST    Editing Messages = PATCH
pub mod discord {
    use surf::{self,http::mime};
    use std::{collections::HashMap,result::*,error::Error};

    use tokio::{self, runtime::Runtime};
    use serde_json::{self,Result,Serializer, Value};
    use serde::{Serialize,Deserialize};


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
        fn send(&self,url: &str) -> WebhookHandle;
        fn edit(&self,message_id: i32,url: &str) -> bool;
    }

    impl WebHook for Webhook {
        //Uses the Struct contents
        fn send(&self,url: &str) -> WebhookHandle {
            let content = serde_json::to_string(&self).unwrap();
            let _url = format!("{}?wait=true",url);
            drop(url);

        
            let response: WebhookHandle = new_rt().block_on(async move {
                let post = surf::post(_url)
                    .body_string(content)
                    .content_type(mime::JSON)
                    .send().await;


                let body = post.expect("Error").body_string().await.unwrap();
                serde_json::from_str(&*body).expect("Couldnt unwrap webhook response idk why")
            });

            return response;
        }

        ///Returns true if status is 200. Use the handle from send
        fn edit(&self,message_id: i32,url: &str) -> bool {
            let content = serde_json::to_string(&self).unwrap();
            let _url = format!("{}/messages/{}",url,message_id);
            drop(url);

            new_rt().block_on(async move {
                let post = surf::patch(_url)
                    .body_string(content)
                    .content_type(mime::JSON)
                    .send().await;

                post.unwrap().status().is_success()
            })
        }
    }

    fn new_rt() -> Runtime {
        tokio::runtime::Builder::new_current_thread()
                .max_blocking_threads(2)
                .on_thread_start(|| { println!("Tokio Thread Started"); })
                .build().expect("Couldnt create tokio runtime")
    }

    pub fn create_webhook(avatar_url: &str,nickname: &str,content: String) -> Webhook {
        Webhook {avatar_url: avatar_url.to_string(), username: nickname.to_string(), content: content }
    }
}



pub mod Components {
    
}
