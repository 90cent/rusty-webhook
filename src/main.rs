use gegake_webhook::discord::*;

pub fn main() {
    let url = "https://discord.com/api/webhooks/896808573491888158/NcUBMrS728DVJUXUxM0ii9LsqzZkYwfXdggvWWQR02Ow33j1x19ow4LxtHhq5Ag40I6Q";
    let webhook = create_webhook("http://aaa.com", "nickname", "test");
    webhook.send(&url);
}

