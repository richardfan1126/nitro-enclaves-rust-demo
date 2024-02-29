use std::collections::HashMap;

use reqwest;
use serde::Deserialize;

const SECRET_API_URL: &str = "https://nitro-enclaves-demo.richardfan.xyz";

#[derive(Deserialize)]
pub struct ApiResponse {
    encrypted_secret: String
}

pub async fn get_encrypted_secret_from_api (attestation_doc_b64: String) -> String {
    let mut map = HashMap::new();
    map.insert("attestation_doc", attestation_doc_b64);

    let client = reqwest::Client::new();

    let response = client.post(SECRET_API_URL)
        .json(&map)
        .send()
        .await
        .expect("Failed to get http response")
        .json::<ApiResponse>()
        .await
        .expect("Failed to parse response");

    response.encrypted_secret
}
