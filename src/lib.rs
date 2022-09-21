pub mod api;
pub mod error;

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn test_post() {
        let creds = api::oauth::PSCredentials::ClientCredentials {
            id: env::var("PS_CLIENT_ID").unwrap().to_string(),
            secret: env::var("PS_CLIENT_SECRET").unwrap().to_string(),
        };
        let mut ps = api::PSServer::new("rocky-ps.allette.com.au".to_string(), creds, None, None);
        println!(
            "{:?}",
            ps.checked_get("/ps/service/groups/test-netdox", None, None)
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        );
    }
}
