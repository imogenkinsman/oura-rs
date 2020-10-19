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

#[derive(Deserialize, Debug)]
struct Sleep {
    sleep: Vec<SleepPeriod>,
}

#[derive(Deserialize, Debug)]
struct SleepPeriod {
    summary_date: String, // change this to parse into date type
    period_id: u8,
    is_longest: u8,
    timezone: i32,
    bedtime_start: String, // should be date
    bedtime_end: String,   // shound be date
    score: u8,
    score_total: u8,
    score_disturbances: u8,
    score_efficiency: u8,
    score_latency: u8,
    score_rem: u8,
    score_deep: u8,
    score_alignment: u8,
    total: u16,
    duration: u16,
    awake: u16,
    light: u16,
    rem: u16,
    deep: u16,
    onset_latency: u16,
    restless: u8,
    efficiency: u8,
    midpoint_time: u16,
    hr_lowest: u16,
    hr_average: f32,
    rmssd: u16,
    breath_average: f32,
    temperature_delta: f32,
    hypnogram_5min: String, // this is a weird one
    hr_5min: Vec<u8>,
    rmssd_5min: Vec<u16>,
}

#[derive(Deserialize, Debug)]
struct Activity {
    activity: Vec<ActivityPeriod>,
}

#[derive(Deserialize, Debug)]
struct ActivityPeriod {}

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

    // public endpoint functions
    // function name | struct it populates | api endpoint
    endpoint!(info, UserInfo, "https://api.ouraring.com/v1/userinfo");
    endpoint!(sleep, Sleep, "https://api.ouraring.com/v1/sleep");
    endpoint!(activity, Activity, "https://api.ouraring.com/v1/activity");
    endpoint!(
        readiness,
        Readiness,
        "https://api.ouraring.com/v1/readiness"
    );
}
