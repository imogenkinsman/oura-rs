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
    client: BasicClient,
}

impl Client {
    pub fn connect(c: ClientConfig) -> Client {
    let client =
    BasicClient::new(
        ClientId::new(c.client_id),
        Some(ClientSecret::new(c.client_secret)),
        AuthUrl::new(c.auth_url)?,
        Some(TokenUrl::new(c.token_url)?)
    )
    .set_redirect_url(RedirectUrl::new(c.redirect_url)?);

    // generate a PKCE challenge - see https://tools.ietf.org/html/rfc7636
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_state`.

// Now you can trade it for an access token.
let token_result = client
    .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
    // Set the PKCE code verifier.
    .set_pkce_verifier(pkce_verifier)
    .request_async(async_http_client)
    .await?;

// // Unwrapping token_result will either produce a Token or a RequestTokenError.

    let client = Client{config: config};
    return client;
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
