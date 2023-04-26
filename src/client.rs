use crate::role::site::Site;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::html;
use serde_json::Value;

/// get_name_info
pub fn get_name_info(resp: Value, name: String) -> Option<Vec<Site>> {
    let mut ret: Vec<Site> = Vec::new();
    let data = resp["data"].as_array()?;
    for i in data {
        let site: Site = serde_json::from_value((*i).clone())
            .unwrap_or_else(|err| panic!("parse site in response \n detail: \n {}", err));

        let ts = site.ts()?;
        for j in ts {
            if j.owner() == name {
                ret.push(Site::new(
                    site.dev_name().to_string(),
                    site.dev_id().to_string(),
                    Some(vec![j.clone()]),
                ));
            }
        }
    }
    if ret.len() == 0 {
        return None;
    }
    return Some(ret);
}

/// get_site_info
pub fn get_site_info(resp: Value) -> Option<Site> {
    let data = resp["data"]
        .as_array()
        .expect("parse data in site response");
    let site = serde_json::from_value(data[0].clone())
        .unwrap_or_else(|err| panic!("parse site in response \n detail: \n {}", err));
    Some(site)
}

/// get_login_info
pub fn get_login_info(resp: Value) -> Option<Student> {
    let data = resp["data"].clone();
    let student: Student = serde_json::from_value(data).unwrap_or_else(|err| panic!("{}", err));
    Some(student)
}

/// get_state_info
pub fn get_state_info(resp: Value) -> Option<Vec<State>> {
    match html::parse_state(resp) {
        Ok(state) => Some(state),
        Err(err) => panic!("{}", err),
    }
}

/// get_cancel_info
pub fn get_cancel_info(resp: Value) -> Option<String> {
    let msg = resp["msg"].as_str();
    match msg {
        Some(msg) => Some(msg.to_string()),
        None => panic!("no msg in cancel response"),
    }
}

/// get_reserve_info
pub fn get_reserve_info(resp: Value) -> Option<String> {
    let msg = resp["msg"].as_str();
    match msg {
        Some(msg) => Some(msg.to_string()),
        None => panic!("no msg in reserve response"),
    }
}

/// get_check_out_info
pub fn get_check_out_info(resp: Value) -> Option<String> {
    let msg = resp["msg"].as_str();
    match msg {
        Some(msg) => Some(msg.to_string()),
        None => panic!("no msg in check out response"),
    }
}
