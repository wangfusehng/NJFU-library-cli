use crate::client;
use crate::utils::*;
use log::*;
use std::collections::HashMap;

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn query_by_name(&self, name: String) -> Option<String> {
        let mut body = HashMap::new();
        body.insert("byType", "devcls");
        body.insert("classkind", "8");
        body.insert("cld_name", "default");
        body.insert("act", "get_rsv_sta");
        let date = time::get_date("%Y-%m-%d");
        body.insert("date", date.as_str());

        for (room_name, room_id) in def::ROOMS.iter() {
            let room_id = room_id.0.to_string();
            let mut data = body.clone();
            data.insert("room_id", room_id.as_str());

            debug!("querying room {}", room_name);
            let resp =
                client::handle_post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), data).ok()?;

            match client::get_name_info(resp, name.clone()) {
                Some(name_info) => {
                    return Some(format!("{}: {}", room_name, name_info));
                }
                None => continue,
            }
        }
        return None;
    }

    pub fn query_by_site(&self, site: String) -> Option<String> {
        let dev_id = floor::get_site_id(site.clone()).to_string();

        let mut body = HashMap::new();
        body.insert("dev_id", dev_id.as_str());
        body.insert("act", "get_rsv_sta");
        let date = time::get_date("%Y-%m-%d");
        body.insert("date", date.as_str());

        let resp =
            client::handle_post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;
        return client::get_site_info(resp);
    }

    pub fn login(&self, username: String, password: String) -> Option<String> {
        let mut body = HashMap::new();
        body.insert("act", "login");
        body.insert("id", username.as_str());
        body.insert("pwd", password.as_str());
        let resp =
            client::handle_post(def::LOGIN_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;

        return client::get_login_info(resp);
    }
}
