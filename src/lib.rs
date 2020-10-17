use serde::Deserialize;
use url::Url;

pub struct Client {
    pub token: String, // personal access token
}

#[derive(Deserialize, Debug)]
struct UserInfo {
    age: u8,
    weight: f32,
    height: u16,
    gender: String,
    email: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    // #[tokio::main]
    pub fn info(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            "https://api.ouraring.com/v1/userinfo",
            &[("access_token", self.token.clone())],
        )?;
        let resp = reqwest::blocking::get(url)?.json::<UserInfo>()?;
        // .json::<HashMap<String, String>>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    // pub fn

    // async fn get(end)
}

/*
client::


*/

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let config = client::ClientConfig{
//             client_id: "foo".to_string(),
//             client_secret: "foo".to_string(),
//             auth_url: "foo".to_string(),
//             token_url: "foo".to_string(),
//             redirect_url: "foo".to_string(),
//         };
//         let response = block_on(client::Client::connect(config));
//         assert!(response.is_ok())
//     }

// #[test]
// fn it_panics() {
//     let client = Client
//     ass
// }
// }
