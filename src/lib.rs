mod client;

/*
client::


*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = client::ClientConfig{
            client_id: "foo".to_string(),
            client_secret: "foo".to_string(),
            auth_url: "foo".to_string(),
            token_url: "foo".to_string(),
            redirect_url: "foo".to_string(),
        };
        let response = block_on(client::Client::connect(config));
        assert!(response.is_ok())
    }

    // #[test]
    // fn it_panics() {
    //     let client = Client
    //     ass
    // }
}
