use crate::client;
use crate::role::info::Info;
use crate::role::site::Site;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::*;
use log::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn query_by_name(&self, name: String) -> Option<Vec<Ts>> {
        let mut body = HashMap::new();
        body.insert("byType", "devcls");
        body.insert("classkind", "8");
        body.insert("cld_name", "default");
        body.insert("act", "get_rsv_sta");
        let date = time::get_date("%Y-%m-%d");
        body.insert("date", date.as_str());

        let mut ret: Vec<Ts> = Vec::new();

        for (room_name, room_id) in def::ROOMS.iter() {
            let room_id = room_id.0.to_string();
            let mut data = body.clone();
            data.insert("room_id", room_id.as_str());

            info!("querying room {}", room_name);
            let resp =
                client::handle_post(def::DEVICE_URL.as_str(), client::HEADERMAP.clone(), data)
                    .expect("net error when querying room");

            match client::get_name_info(resp, name.clone()) {
                Some(info) => {
                    ret.append(info.into_iter().collect::<Vec<Ts>>().as_mut());
                }
                None => {}
            }
        }
        if ret.len() == 0 {
            return None;
        }
        return Some(ret);
    }

    pub fn query_by_site(&self, site: String) -> Option<Site> {
        let dev_id = floor::get_site_id(site.clone());
        match dev_id {
            Ok(dev_id) => {
                let mut body = HashMap::new();
                body.insert("dev_id", dev_id.as_str());
                body.insert("act", "get_rsv_sta");
                let date = time::get_date("%Y-%m-%d");
                body.insert("date", date.as_str());

                let resp =
                    client::handle_post(def::DEVICE_URL.as_str(), client::HEADERMAP.clone(), body)
                        .expect("net error when querying site");

                client::get_site_info(resp)
            }
            Err(e) => {
                error!("error when querying site: {}", e);
                return None;
            }
        }
    }

    fn handle_login(&self) -> Option<Student> {
        let mut student = Info::new("".to_string(), "".to_string());
        student.read_from_file().expect("read student info failed");

        let mut body = HashMap::new();
        body.insert("act", "login");
        body.insert("id", student.username());
        body.insert("pwd", student.password());
        let resp = client::handle_post(def::LOGIN_URL.as_str(), client::HEADERMAP.clone(), body)
            .expect("net error when login");

        client::get_login_info(resp)
    }

    pub fn login(&self, username: String, password: String) -> Option<Student> {
        let student = Info::new(username.clone(), password.clone());
        student.save_to_file().expect("save student info failed");
        self.handle_login()
    }

    pub fn status(&self) -> Option<Vec<State>> {
        //login
        self.handle_login();

        let mut body = HashMap::new();
        body.insert("act", "get_History_resv");
        body.insert("strat", "90");
        body.insert("StatFlag", "New");

        let resp =
            client::handle_post(def::CENTER_URL.as_str(), client::HEADERMAP.clone(), body).ok()?;

        client::get_state_info(resp)
    }

    pub fn cancel(&self, id: String) -> Option<String> {
        //login
        self.handle_login();

        let mut body = HashMap::new();
        body.insert("act", "del_resv");
        body.insert("id", id.as_str());

        let resp =
            client::handle_post(def::RESERVE_URL.as_str(), client::HEADERMAP.clone(), body).ok()?;

        client::get_cancel_info(resp)
    }

    pub fn reserve(&self, site: String, day: String, start: String, end: String) -> Option<String> {
        //login
        self.handle_login();

        let id = floor::get_site_id(site);
        match id {
            Ok(id) => {
                let mut body = HashMap::new();
                body.insert("act", "set_resv");
                body.insert("dev_id", id.as_str());
                let start_time = format!("{} {}", day, start);
                let end_time = format!("{} {}", day, end);
                body.insert("start", start_time.as_str());
                body.insert("end", end_time.as_str());
                
                let resp =
                    client::handle_post(def::RESERVE_URL.as_str(), client::HEADERMAP.clone(), body);
                client::get_reserve_info(resp.ok()?)
            }
            Err(e) => {
                error!("get site id error: {}", e);
                None
            }
        }
    }
}
