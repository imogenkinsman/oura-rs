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
    readiness: Vec<ReadinessPeriod>,
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

macro_rules! endpoint {
    ($name:ident, $type:ty, $url:literal) => {
        pub fn $name(&self) -> Result<(), Box<dyn std::error::Error>> {
            let url = Url::parse_with_params($url, &[("access_token", self.token.clone())])?;
            let resp = reqwest::blocking::get(url)?.json::<$type>()?;
            println!("{:#?}", resp);
            Ok(())
        }
    };
}

impl Client {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    endpoint!(info, UserInfo, "https://api.ouraring.com/v1/userinfo");
    endpoint!(
        readiness,
        Readiness,
        "https://api.ouraring.com/v1/readiness"
    );

    // pub fn
}
