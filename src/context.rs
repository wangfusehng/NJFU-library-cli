use crate::client;
use crate::utils::student::Student;
use crate::utils::*;
use log::*;
use std::collections::HashMap;

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn query_by_name(&self, name: String) -> Option<(String, String, String, String)> {
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

            info!("querying room {}", room_name);
            let resp =
                client::handle_post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), data).ok()?;

            match client::get_name_info(resp, name.clone()) {
                Some(info) => return Some(info),
                None => continue,
            }
        }
        return None;
    }

    pub fn query_by_site(&self, site: String) -> Option<site::Site> {
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

    pub fn login(&self, username: String, password: String) -> Option<(String, String, String)> {
        let student = Student::new(username.clone(), password.clone());
        student.save_to_file().ok()?;

        let mut body = HashMap::new();
        body.insert("act", "login");
        body.insert("id", username.as_str());
        body.insert("pwd", password.as_str());
        let resp =
            client::handle_post(def::LOGIN_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;

        return client::get_login_info(resp);
    }

    pub fn status(& self) -> Option<(String, String, String,String)> {
        let mut student = Student::new("".to_string(), "".to_string());
        student.read_from_file().ok()?;

        //login
        let mut body = HashMap::new();
        body.insert("act", "login");
        body.insert("id", student.username());
        body.insert("pwd", student.password());
        client::handle_post(def::LOGIN_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;

        let mut body = HashMap::new();
        body.insert("act", "get_History_resv");
        body.insert("strat", "90");
        body.insert("StatFlag", "New");

        let resp =
            client::handle_post(def::CENTER_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;

        return client::get_status_info(resp);
    }
}
