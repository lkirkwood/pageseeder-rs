pub mod api;
pub mod error;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test() {
        let creds = api::oauth::PSCredentials::ClientCredentials {
            id: "BC45D622FDEF5355".to_string(),
            secret: "KQPx11bHK0H4nekcxOv9sA".to_string(),
        };
        let mut ps =
            api::BlockingPSServer::new("rocky-ps.allette.com.au".to_string(), creds, None, None);
    }

    #[test]
    fn test_post() {
        let creds = api::oauth::PSCredentials::ClientCredentials {
            id: "BC45D622FDEF5355".to_string(),
            secret: "KQPx11bHK0H4nekcxOv9sA".to_string(),
        };
        let mut ps =
            api::BlockingPSServer::new("rocky-ps.allette.com.au".to_string(), creds, None, None);
        println!(
            "{:?}",
            ps.checked_get("/ps/service/groups/test-netdox", None, None)
                .unwrap()
                .text()
                .unwrap()
        );
    }
}
