use std::{collections::HashMap, env};

use super::{oauth::PSCredentials, PSServer};

fn credentials() -> PSCredentials {
    PSCredentials::ClientCredentials {
        id: env::var("PS_TEST_ID").expect("Set environment variable PS_TEST_ID"),
        secret: env::var("PS_TEST_SECRET").expect("Set environment variable PS_TEST_SECRET"),
    }
}

fn test_server() -> PSServer {
    PSServer::new(
        env::var("PS_TEST_URL").expect("Set environment variable PS_TEST_URL"),
        credentials(),
    )
}

#[tokio::test]
async fn test_group_search() {
    test_server()
        .group_search(
            &env::var("PS_TEST_GROUP").unwrap(),
            HashMap::from([("question", "config")]),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_uri_history() {
    test_server()
        .get_uri_history(&env::var("PS_TEST_GROUP").unwrap(), "_nd_config")
        .await
        .unwrap();
}
