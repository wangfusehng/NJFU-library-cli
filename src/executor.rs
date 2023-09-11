use super::config::{self, Config};
use super::def;
use crate::cli::reserve;
use crate::role::dev::Dev;
use crate::role::resp::Data;
use crate::role::resp::Resp;
use crate::role::resv::Resv;
use crate::role::site::*;
use crate::role::user::*;
use crate::utils::*;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

pub fn query_by_name(day: u32, name: String) -> Result<Resp> {
    let date = time::get_date_with_offset("%Y%m%d", day);

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(200));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.dim.bold} {prefix:.bold.dim} query floor: {wide_msg}",
        )
        .context("Failed to set progress bar style.")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    let total = def::FLOOR.len() as u64;

    for (pos, (floor_name, floor)) in def::ROOMS.iter().enumerate() {
        pb.set_prefix(format!("[{}/{}]", pos, total));
        pb.set_message(floor_name.to_string());

        let room_id = floor.room_id().to_string();
        let url = format!(
            "{}?roomIds={}&resvDates={}&sysKind=8",
            def::QUERY_URL,
            room_id,
            date
        );
        let resp = def::CLIENT.get(url).send()?.json::<Resp>()?;
        match handle_resp::get_name_info(resp, name.clone()) {
            Ok(ret) => {
                pb.finish_and_clear();
                return Ok(ret);
            }
            Err(_) => continue,
        }
    }

    pb.finish_and_clear();
    Err(anyhow!("no user can be found"))
}

pub fn query_by_site(day: u32, site: String) -> Result<Resp> {
    let floor = name_to_floor(site.clone())?;
    let room_id = floor.room_id().to_string();
    let site_id = name_to_id(site.clone())?;
    let site_index = site_id - floor.dev_start() + 1;
    let date = time::get_date_with_offset("%Y%m%d", day);

    let url = format!(
        "{}?roomIds={}&resvDates={}&sysKind=8",
        def::QUERY_URL,
        room_id,
        date
    );
    let resp = def::CLIENT.get(url).send()?.json::<Resp>()?;
    handle_resp::get_site_info(resp, site_index)
}

/// login to the server.
pub fn login(username: String, password: String, cookie: String) -> Result<Config> {
    let config = Config::new(username, password, cookie, None);
    config::save_config_to_file(&config).context("save cookie failed")?;
    Ok(config)
}

/// query the user status.
pub fn state() -> Result<Resp> {
    let begin_date = time::get_date_with_offset("%Y-%m-%d", 0);
    let end_date = time::get_date_with_offset("%Y-%m-%d", 2);
    let url = format!(
        "{}?beginDate={}&endDate={}",
        def::RESV_INFO_URL,
        begin_date,
        end_date
    );
    Ok(def::CLIENT.get(url).send()?.json::<Resp>()?)
}

/// cancel the reservation.
pub fn cancel(uuid: String) -> Result<Resp> {
    let mut body = HashMap::new();
    body.insert("uuid", uuid.as_str());

    Ok(def::CLIENT
        .post(def::CANCEL_URL)
        .json(&body)
        .send()?
        .json::<Resp>()?)
}

pub fn reserve(
    sites: Option<Vec<String>>,
    filter: Vec<String>,
    day: u32,
    start: String,
    end: String,
    retry: u32,
) -> Result<Resp> {
    let config = config::load_config_from_file()?;
    let user = search_user_info(&config)?;
    let appacc_no = user.accno().parse::<u32>()?;
    let site_list = sites.unwrap_or(
        // random
        {
            let mut cnt = retry;
            let mut ret: Vec<String> = Vec::new();
            while cnt > 0 {
                ret.push(get_random_site_name()?);
                cnt -= 1;
            }
            ret
        },
    );

    for site in site_list {
        // filter by floor
        if !(site_fiter_by_floor(site.clone(), filter.clone())?) {
            continue;
        }
        println!("try to reserve {}", site);
        let ret = handle_reserve(site.clone(), appacc_no, day, start.clone(), end.clone());
        match ret {
            Ok(ret) => return Ok(ret),
            Err(_) => continue,
        }
    }
    Err(anyhow!("no site from put in can be reserved"))
}

fn handle_reserve(
    site: String,
    appacc_no: u32,
    day: u32,
    start: String,
    end: String,
) -> Result<Resp> {
    let date = time::get_date_with_offset("%Y-%m-%d", day);
    let start_time = format!("{} {}:00", date, start);
    let end_time = format!("{} {}:00", date, end);
    let resv_dev = name_to_id(site.clone())?;

    let data = serde_json::json!({
        "sysKind": 8,
        "appAccNo": appacc_no,
        "resvMember": [ appacc_no ],
        "resvBeginTime": start_time,
        "resvEndTime": end_time,
        "resvDev": [ resv_dev ],
    });

    Ok(def::CLIENT
        .post(def::RESERVE_URL)
        .json(&data)
        .send()?
        .json::<Resp>()?)
}
