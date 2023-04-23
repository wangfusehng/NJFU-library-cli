use crate::utils::*;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
    pub static ref CLIENT: reqwest::blocking::Client = {
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();

        client
    };
}

pub fn handle_post(
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

pub fn get_name_info(resp: Value, name: String) -> Option<(String, String, String, String)> {
    let p = resp["data"].as_array()?;
    for i in p {
        if i["ts"][0]["owner"] == name.as_str() {
            return Some((
                i["name"].as_str()?.to_string(),
                i["ts"][0]["owner"].as_str()?.to_string(),
                i["ts"][0]["start"].as_str()?.to_string(),
                i["ts"][0]["end"].as_str()?.to_string(),
            ));
        }
    }
    return None;
}

pub fn get_site_info(resp: Value) -> Option<site::Site> {
    let data = resp["data"].as_array()?;
    let dev_name = data[0]["devName"].as_str()?.to_string();
    let dev_id = data[0]["devId"].as_str()?.to_string();
    let ts = data[0]["ts"].as_array()?;
    let user_list: Option<Vec<_>> = Some(
        ts.iter()
            .map(|x| {
                (
                    x["owner"].to_string(),
                    x["start"].to_string(),
                    x["end"].to_string(),
                    x["status"].to_string(),
                )
            })
            .collect(),
    );

    Some(site::Site::new(dev_name, dev_id, user_list))
}

pub fn get_login_info(resp: Value) -> Option<(String, String, String)> {
    let id = resp["data"]["id"].as_str()?.to_string();
    let name = resp["data"]["name"].as_str()?.to_string();
    let dept = resp["data"]["dept"].as_str()?.to_string();
    Some((id, name, dept))
}

pub fn get_status_info(
    resp: Value,
) -> Option<(String, String, String,String)> {
    let msg = Html::parse_fragment(resp["msg"].as_str()?);

    let a_selector = Selector::parse(".box a").ok()?;
    let time_selector = Selector::parse(".text-primary").ok()?;
    let id_selector = Selector::parse(".click").ok()?;

    let site = msg.select(&a_selector).nth(0)?.inner_html();
    let start_time = msg.select(&time_selector).nth(0)?.inner_html();
    let end_time = msg.select(&time_selector).nth(1)?.inner_html();
    let id_class = msg.select(&id_selector).nth(0)?.html().to_string();
    let id = id_class
        .split("id=")
        .nth(1)?
        .split("\"")
        .nth(1)?
        .to_string();

    Some((site, id, start_time,end_time))
}
