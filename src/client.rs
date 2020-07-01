use oauth2::{
    AsyncCodeTokenRequest,
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use url::Url;

pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn connect(config: ClientConfig) -> Client {
        let client = Client{config: config};
        return client;
    }
}

pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
//
//     #[test]
//     fn it_panics() {
//         let client = Client
//         ass
//     }
// }
