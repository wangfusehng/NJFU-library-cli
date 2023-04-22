use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    pub static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub fn handle_post(
    url: &str,
    headers: HeaderMap,
    body: HashMap<&str, &str>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = CLIENT
        .post(url)
        .headers(headers)
        .form(&body)
        .send()?
        .json::<serde_json::Value>()?;
    Ok(resp)
}

pub fn get_name_info(resp: Value, name: String) -> Option<String> {
    let p = resp["data"].as_array()?;
    for i in p {
        if i["ts"][0]["owner"] == name.as_str() {
            return Some(format!(
                "{} {} start:{} end:{}",
                i["name"], i["ts"][0]["owner"], i["ts"][0]["start"], i["ts"][0]["end"]
            ));
        }
    }
    return None;
}

pub fn get_site_info(resp: Value) -> Option<String> {
    let p = resp["data"].as_array().unwrap();
    let site = p[0].to_owned();
    let ts = site["ts"].as_array().unwrap();

    let mut result: String = String::new();

    if ts.len() != 0 {
        for i in ts {
            result.push_str(
                format!(
                    "site:{} owner:{} start:{} end:{}",
                    site["name"], i["owner"], i["start"], i["end"]
                )
                .as_str(),
            );
        }
    }
    if result.is_empty() {
        return None;
    } else {
        return Some(result);
    }
}

pub fn get_login_info(resp: Value) -> Option<String> {
    if let true = resp["data"].is_null() {
        return Some(resp["msg"].as_str()?.to_string());
    }
    let p = &resp["data"];
    let id = p["id"].as_str().unwrap();
    let name = p["name"].as_str().unwrap();
    let dept = p["dept"].as_str().unwrap();

    if id.is_empty() || name.is_empty() || dept.is_empty() {
        return None;
    } else {
        Some(format!("{} {} {}", id, name, dept))
    }
}
