use crate::role::site::Site;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::html;
use anyhow::{anyhow, Context, Result};
use reqwest::blocking::Response;

/// get_name_info
pub fn get_name_info(resp: Response, name: String) -> Result<Vec<Site>> {
    let resp = resp.json::<serde_json::Value>()?;
    let mut ret: Vec<Site> = Vec::new();
    let data = resp["data"]
        .as_array()
        .context("parse site in get_name_info error")?;
    for i in data {
        let site: Site = serde_json::from_value((*i).clone())?;

        let ts = site.ts().context("parse ts in get_name_info error")?;
        for j in ts {
            if j.owner() == name {
                ret.push(Site::new(
                    site.dev_name().to_string(),
                    site.dev_id().to_string(),
                    site.lab_id().to_string(),
                    Some(vec![j.clone()]),
                ));
            }
        }
    }
    Ok(ret)
}

/// get_site_info
pub fn get_site_info(resp: Response) -> Result<Site> {
    let resp = resp.json::<serde_json::Value>()?;
    let data = resp["data"].as_array().context("parse site in response")?;

    serde_json::from_value(data[0].clone()).context("parse site error")
}

/// get_login_info
pub fn get_login_info(resp: Response) -> Result<Student> {
    let resp = resp.json::<serde_json::Value>()?;
    let data = resp["data"].clone();
    if let serde_json::Value::Null = data {
        Err(anyhow!(resp["msg"].as_str().unwrap().to_string()))
    } else {
        let student: Student = serde_json::from_value(data).context("parse student error")?;
        Ok(student)
    }
}

/// get_state_info
pub fn get_state_info(resp: Response) -> Result<Vec<State>> {
    let resp = resp.json::<serde_json::Value>()?;
    let msg = resp["msg"]
        .as_str()
        .context("no msg in response")?
        .to_string();

    html::parse_state(msg)
}

/// get_cancel_info
pub fn get_cancel_info(resp: Response) -> Result<String> {
    // NOTE: the response is not a stand json
    //       the response contains two json gist,so we need to cut it
    let mut resp = resp.text()?;
    resp.truncate(resp.find("}").context("parse cancel info error")? + 1);
    let resp = serde_json::from_str::<serde_json::Value>(&resp)?;
    let ret = resp["msg"]
        .as_str()
        .context("parse msg in cancel response error")?
        .to_string();
    if ret.contains("成功") {
        Ok(ret)
    } else {
        Err(anyhow!(ret))
    }
}

/// get_reserve_info
pub fn get_reserve_info(resp: Response) -> Result<String> {
    let resp = resp.json::<serde_json::Value>()?;
    let ret = resp["msg"]
        .as_str()
        .context("parse msg in reserve response error")?
        .to_string();
    if ret.contains("成功") {
        Ok(ret)
    } else {
        Err(anyhow!(ret))
    }
}

/// get_check_out_info
pub fn get_check_out_info(resp: Response) -> Result<String> {
    // NOTE: the response is not a stand json
    //       the response contains two json gist,so we need to cut it
    let mut resp = resp.text()?;
    resp.truncate(resp.find("}").context("parse check out info error")? + 1);
    let resp = serde_json::from_str::<serde_json::Value>(&resp)?;
    let ret = resp["msg"]
        .as_str()
        .context("parse msg in check out response error")?
        .to_string();
    if ret.contains("成功") {
        Ok(ret)
    } else {
        Err(anyhow!(ret))
    }
}
