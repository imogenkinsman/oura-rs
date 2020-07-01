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
        let client = client::Client::connect(config);
        // let client = client::Client
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn it_panics() {
    //     let client = Client
    //     ass
    // }
}
