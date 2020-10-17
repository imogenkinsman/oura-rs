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

#[derive(Deserialize, Debug)]
struct Readiness {
    summary_date: String, // change this to parse into date type
    period_id: u8,
    score: u8,
    score_previous_night: u8,
    score_sleep_balance: u8,
    score_previous_day: u8,
    score_activity_balance: u8,
    score_resting_hr: u8,
    score_hrv_balance: u8,
    score_recovery_index: u8,
    score_temperature: u8,
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
