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
    readiness_periods: Vec<ReadinessPeriod>,
}

#[derive(Deserialize, Debug)]
struct ReadinessPeriod {
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
    const USER_INFO_URL: &'static str = "https://api.ouraring.com/v1/readiness";
    const READINESS_URL: &'static str = "https://api.ouraring.com/v1/userinfo";

    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn info(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            Client::USER_INFO_URL,
            &[("access_token", self.token.clone())],
        )?;
        let resp = reqwest::blocking::get(url)?.json::<UserInfo>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    pub fn readiness(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            Client::READINESS_URL,
            &[("access_token", self.token.clone())],
        )?;
        let resp = reqwest::blocking::get(url)?.json::<UserInfo>()?;
        println!("{:#?}", resp);
        Ok(())
    }

    // fn make_request(
    //     &self,
    //     endpoint: &str,
    // ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    //     let url = format!("{}{}", Client::BASE_URL, endpoint);
    //     let url = Url::parse_with_params(url, &[("access_token", self.token.clone())])?;
    //     Ok(reqwest::blocking::get(url)?)
    //     // Ok(())
    // }
}
