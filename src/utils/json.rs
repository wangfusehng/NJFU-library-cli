use crate::role::site::Site;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::html;
use anyhow::{anyhow, Context, Result};
use serde_json::Value;

/// get_name_info
pub fn get_name_info(resp: Value, name: String) -> Result<Vec<Site>> {
    let mut ret: Vec<Site> = Vec::new();
    let data = resp["data"].as_array().context("parse site in response")?;
    for i in data {
        let site: Site = serde_json::from_value((*i).clone())?;

        let ts = site.ts().ok_or(anyhow!("parse ts in site"))?;
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
pub fn get_site_info(resp: Value) -> Result<Site> {
    let data = resp["data"].as_array().context("parse site in response")?;

    serde_json::from_value(data[0].clone()).context("parse site error")
}

/// get_login_info
pub fn get_login_info(resp: Value) -> Result<Student> {
    let data = resp["data"].clone();
    match data {
        Value::Null => Err(anyhow!(resp["msg"].as_str().unwrap().to_string())),
        _ => {
            let student: Student = serde_json::from_value(data).context("parse student error")?;
            Ok(student)
        }
    }
}

/// get_state_info
pub fn get_state_info(resp: Value) -> Result<Vec<State>> {
    html::parse_state(resp)
}

/// get_cancel_info
pub fn get_cancel_info(resp: Value) -> Result<String> {
    resp["msg"]
        .as_str()
        .map(|s| s.to_string())
        .context("parse cancel info error")
}

/// get_reserve_info
pub fn get_reserve_info(resp: Value) -> Result<String> {
    resp["msg"]
        .as_str()
        .map(|s| s.to_string())
        .context("parse reserve info error")
}

/// get_check_out_info
pub fn get_check_out_info(resp: Value) -> Result<String> {
    resp["msg"]
        .as_str()
        .map(|s| s.to_string())
        .context("parse check out info error")
}
