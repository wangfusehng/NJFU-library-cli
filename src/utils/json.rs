use crate::role::site::Site;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::html;
use serde_json::Value;

/// get_name_info
pub fn get_name_info(resp: Value, name: String) -> Result<Vec<Site>, Box<dyn std::error::Error>> {
    let mut ret: Vec<Site> = Vec::new();
    let data = resp["data"].as_array().ok_or("parse site in response")?;
    for i in data {
        let site: Site = serde_json::from_value((*i).clone())?;

        let ts = site.ts().ok_or("parse ts in site")?;
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
    Ok(ret)
}

/// get_site_info
pub fn get_site_info(resp: Value) -> Result<Site, String> {
    let data = resp["data"]
        .as_array()
        .ok_or("parse site in response".to_string())?;

    serde_json::from_value(data[0].clone()).unwrap_or(Err("parse site in response".to_string()))
}

/// get_login_info
pub fn get_login_info(resp: Value) -> Result<Student, String> {
    let data = resp["data"].clone();
    match data {
        Value::Null => Err(resp["msg"].as_str().unwrap().to_string()),
        _ => {
            let student: Student =
                serde_json::from_value(data).unwrap_or_else(|err| panic!("{}", err));
            Ok(student)
        }
    }
}

/// get_state_info
pub fn get_state_info(resp: Value) -> Result<Vec<State>, String> {
    match html::parse_state(resp) {
        Ok(state) => Ok(state),
        Err(err) => Err(err.to_string()),
    }
}

/// get_cancel_info
pub fn get_cancel_info(resp: Value) -> Result<String, String> {
    resp["msg"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("no msg in cancel response".to_string())
}

/// get_reserve_info
pub fn get_reserve_info(resp: Value) -> Result<String, String> {
    resp["msg"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("no msg in reserve response".to_string())
}

/// get_check_out_info
pub fn get_check_out_info(resp: Value) -> Result<String, String> {
    resp["msg"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("no msg in check out response".to_string())
}
