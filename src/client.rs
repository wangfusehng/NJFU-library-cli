use crate::role::site::Site;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;

lazy_static! {
/// rewest header
    pub static ref HEADERMAP: reqwest::header::HeaderMap = {
        let mut headermap = reqwest::header::HeaderMap::new();
        headermap.insert(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36"
                .parse()
                .unwrap(),
        );
        headermap.insert(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headermap.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("private"),
        );
        headermap
    };
    pub static ref CLIENT: reqwest::blocking::Client = {
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();

        client
    };
}

/// # reqwest post
///
/// # params:
/// - url: &str
/// - headers: HeaderMap
/// - body: HashMap<&str, &str>
///
/// # return:
/// - Result<Value, reqwest::Error>
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

/// # get_name_info
///
/// # params:
/// - resp: Value
/// - name: String
///
/// # return:
/// - Option<Vec<Ts>>

pub fn get_name_info(resp: Value, name: String) -> Option<Vec<Ts>> {
    let mut ret: Vec<Ts> = Vec::new();
    let p = resp["data"].as_array()?;
    for i in p {
        let ts = i["ts"].as_array()?;
        if ts.len() == 0 {
            continue;
        }
        for j in ts {
            let owner = j["owner"].as_str()?.to_string();
            if owner == name {
                let start = j["start"].as_str()?.to_string();
                let end = j["end"].as_str()?.to_string();
                let state = j["state"].as_str()?.to_string();
                ret.push(Ts::new(owner.clone(), start, end, state));
            }
        }
    }
    if ret.len() == 0 {
        return None;
    }
    return Some(ret);
}

/// # get_site_info
///
/// # params:
/// - resp: Value
///
/// # return:
/// - Option<Site>
pub fn get_site_info(resp: Value) -> Option<Site> {
    let data = resp["data"].as_array()?;
    let dev_name = data[0]["devName"].as_str()?.to_string();
    let dev_id = data[0]["devId"].as_str()?.to_string();
    let ts = data[0]["ts"].as_array()?;
    let user_list: Option<Vec<_>> = Some(
        ts.iter()
            .map(|x| {
                Ts::new(
                    x["owner"].to_string(),
                    x["start"].to_string(),
                    x["end"].to_string(),
                    x["status"].to_string(),
                )
            })
            .collect(),
    );

    Some(Site::new(dev_name, dev_id, user_list))
}

/// # get_login_info
/// get login info from response
///
/// # params:
/// - resp: Value
///
/// # return:
/// - Option<Student>
pub fn get_login_info(resp: Value) -> Option<Student> {
    let id = resp["data"]["id"].as_str()?.to_string();
    let name = resp["data"]["name"].as_str()?.to_string();
    Some(Student::new(id, name))
}

/// # get_state_info
/// get state info from response
///
/// # params:
/// - resp: Value
///
/// # return:
/// - Option<Vec<State>>

pub fn get_state_info(resp: Value) -> Option<Vec<State>> {
    let msg = Html::parse_fragment(resp["msg"].as_str()?);
    let context_selector = Selector::parse(".box").ok()?;
    let context = msg.select(&context_selector);

    let mut ret: Vec<State> = Vec::new();

    for item in context {
        let a_selector = Selector::parse(".box a").ok()?;
        let time_selector = Selector::parse(".text-primary").ok()?;
        let id_selector = Selector::parse(".click").ok()?;
        let site = item.select(&a_selector).nth(0)?.inner_html();
        let start_time = item.select(&time_selector).nth(0)?.inner_html();
        let end_time = item.select(&time_selector).nth(1)?.inner_html();
        let id_class = item.select(&id_selector).nth(0)?.html().to_string();
        let id = id_class
            .split("id=")
            .nth(1)?
            .split("\"")
            .nth(1)?
            .to_string();

        ret.push(State::new(id, site, start_time, end_time))
    }
    Some(ret)
}

/// # get_cancel_info
/// get cancel info from response
///
/// # params:
/// - resp: Value
/// # return:
/// - Option<String>

pub fn get_cancel_info(resp: Value) -> Option<String> {
    
    let msg = resp["msg"].as_str()?.to_string();
    Some(msg)
}
