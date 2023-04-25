use crate::client;
use crate::role::login::Login;
use crate::role::site::*;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::*;
use log::*;
use std::collections::HashMap;

/// The context of the program.
#[derive(Debug)]
pub struct Context {}

impl Context {
    /// # Context constructor.
    /// Create a new context.
    pub fn new() -> Self {
        Self {}
    }

    /// # Query the information of a student.
    pub fn query_by_name(&self, name: String) -> Option<Vec<Site>> {
        let mut body = HashMap::new();
        body.insert("byType", "devcls");
        body.insert("classkind", "8");
        body.insert("cld_name", "default");
        body.insert("act", "get_rsv_sta");
        let date = time::get_date("%Y-%m-%d");
        body.insert("date", date.as_str());

        let mut ret: Vec<Site> = Vec::new();

        for (room_name, floor) in def::ROOMS.iter() {
            let room_id = floor.room_id().to_string();
            let mut data = body.clone();
            data.insert("room_id", room_id.as_str());

            info!("querying room {}", room_name);
            let resp = http::post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), data)
                .unwrap_or_else(|err| {
                    panic!(
                        "parse error when scan student in {} \n detail: {:?}",
                        room_name, err
                    )
                });

            match client::get_name_info(resp, name.clone()) {
                Some(info) => {
                    ret.append(info.into_iter().collect::<Vec<Site>>().as_mut());
                }
                None => {}
            }
        }
        if ret.len() == 0 {
            return None;
        }
        return Some(ret);
    }

    /// # Query the information of a site.
    pub fn query_by_site(&self, site: String) -> Option<Site> {
        let dev_id = get_site_id(site.clone());
        match dev_id {
            Ok(dev_id) => {
                let mut body = HashMap::new();
                body.insert("dev_id", dev_id.as_str());
                body.insert("act", "get_rsv_sta");
                let date = time::get_date("%Y-%m-%d");
                body.insert("date", date.as_str());

                let resp = http::post(def::DEVICE_URL.as_str(), def::HEADERMAP.clone(), body)
                    .expect("net error when querying site");

                client::get_site_info(resp)
            }
            Err(e) => {
                error!("error when querying site: {}", e);
                return None;
            }
        }
    }

    /// # handle actual login to the server.
    fn handle_login(&self) -> Option<Student> {
        let mut login = Login::new("".to_string(), "".to_string());
        login.read_from_file().expect("read student info failed");

        let mut body = HashMap::new();
        body.insert("act", "login");
        body.insert("id", login.username());
        body.insert("pwd", login.password());
        let resp = http::post(def::LOGIN_URL.as_str(), def::HEADERMAP.clone(), body)
            .expect("net error when login");

        client::get_login_info(resp)
    }

    /// # login to the server.
    pub fn login(&self, username: String, password: String) -> Option<Student> {
        let student = Login::new(username.clone(), password.clone());
        student.save_to_file().expect("save student info failed");
        self.handle_login()
    }

    /// # query the user status.
    pub fn status(&self) -> Option<Vec<State>> {
        //login
        self.handle_login();

        let mut body = HashMap::new();
        body.insert("act", "get_History_resv");
        body.insert("strat", "90");
        body.insert("StatFlag", "New");

        let resp = http::post(def::CENTER_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;

        client::get_state_info(resp)
    }

    /// # cancel the reservation.
    pub fn cancel(&self, id: String) -> Option<String> {
        //login
        self.handle_login();

        let mut body = HashMap::new();
        body.insert("act", "del_resv");
        body.insert("id", id.as_str());

        let resp = http::post(def::RESERVE_URL.as_str(), def::HEADERMAP.clone(), body).ok()?;

        client::get_cancel_info(resp)
    }

    /// # reserve the site.
    pub fn reserve(&self, site: String, day: String, start: String, end: String) -> Option<String> {
        //login
        self.handle_login();

        let id = get_site_id(site);
        match id {
            Ok(id) => {
                let mut body = HashMap::new();
                body.insert("act", "set_resv");
                body.insert("dev_id", id.as_str());
                let start_time = format!("{} {}", day, start);
                let end_time = format!("{} {}", day, end);
                body.insert("start", start_time.as_str());
                body.insert("end", end_time.as_str());

                let resp = http::post(def::RESERVE_URL.as_str(), def::HEADERMAP.clone(), body);
                client::get_reserve_info(resp.ok()?)
            }
            Err(e) => {
                error!("get site id error: {}", e);
                None
            }
        }
    }
}
