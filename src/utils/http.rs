use super::def::CLIENT;
use anyhow::anyhow;
use anyhow::Result;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::{collections::HashMap, io::Read};

/// reqwest post
pub fn post(url: &str, headers: HeaderMap, body: HashMap<&str, &str>) -> Result<Value> {
    let resp = CLIENT
        .post(url)
        .headers(headers.clone())
        .form(&body)
        .send()?
        .json::<serde_json::Value>();

    // try to decode json
    match resp {
        Ok(resp) => Ok(resp.clone()),
        Err(e) => {
            if e.is_decode() {
                let resp = CLIENT
                    .post(url)
                    .headers(headers)
                    .form(&body)
                    .send()?
                    .text()?;
                // try to decode json with error format
                let pos = resp.find('}').unwrap_or(0);
                let resp = &resp[..pos + 1];
                serde_json::from_str(resp).map_err(|e| anyhow!(e))
            } else {
                // other error
                Err(e.into())
            }
        }
    }
}
