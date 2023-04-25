use reqwest::header::HeaderMap;
use std::collections::HashMap;
use serde_json::Value;
use super::def::CLIENT;

/// reqwest post
pub fn post(
    url: &str,
    headers: HeaderMap,
    body: HashMap<&str, &str>,
) -> Result<Value, reqwest::Error> {
    let resp = CLIENT
        .post(url)
        .headers(headers)
        .form(&body)
        .send()?
        .json::<serde_json::Value>()?;
    Ok(resp)
}
