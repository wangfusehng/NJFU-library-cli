use super::def;
use crate::error::RespError;

use crate::njfulib::config::{self, Config};
use crate::njfulib::resp::Data;
use crate::njfulib::resp::Resp;
use crate::njfulib::site::*;
use crate::utils::handle::{self, handle_status};
use crate::utils::{filter::handle_filter, *};
use anyhow::{anyhow, Context, Result};

use indicatif::{ProgressBar, ProgressStyle};

use std::collections::HashMap;
use std::time::Duration;

pub async fn login(username: String, password: String, cookie: String) -> Result<Resp> {
    let config = Config::new(username, password, cookie, None);
    config::save_config_to_file(&config)
}

pub async fn query_by_name(day: u32, name: String, filter: Option<Vec<String>>) -> Result<Resp> {
    let date = time::get_date_with_offset("%Y%m%d", day as i32);

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.dim.bold} {prefix:.bold.dim} query floor: {wide_msg}",
        )
        .context("Failed to set progress bar style.")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    let filter = filter::handle_filter(filter)?;
    let total = filter.len();

    for (pos, floor) in filter.iter().enumerate() {
        pb.set_prefix(format!("[{}/{}]", pos, total));
        pb.set_message(floor.room_name.to_string());

        let room_id = floor.room_id.to_string();
        let url = format!(
            "{}?roomIds={}&resvDates={}&sysKind=8",
            def::QUERY_URL,
            room_id,
            date
        );
        let resp = def::CLIENT.get(url).send().await?.json::<Resp>().await?;
        match handle::get_name_info(resp, name.clone()).await {
            Ok(ret) => {
                if ret.code == 0 {
                    pb.finish_and_clear();
                    return Ok(ret);
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        }
    }

    pb.finish_and_clear();
    Err(anyhow!(RespError::Nodata))
}

pub async fn query_by_site(day: u32, site: String) -> Result<Resp> {
    let floor = site_name_to_floor(site.clone())?;
    let room_id = floor.room_id.to_string();
    let site_id = site_name_to_id(site.clone())?;
    let site_index = site_id - floor.dev_start + 1;
    let date = time::get_date_with_offset("%Y%m%d", day as i32);

    let url = format!(
        "{}?roomIds={}&resvDates={}&sysKind=8",
        def::QUERY_URL,
        room_id,
        date
    );
    let resp = def::CLIENT.get(url).send().await?.json::<Resp>().await?;
    handle::get_site_info(resp, site_index).await
}

pub async fn state(day: u32) -> Result<Resp> {
    let begin_date = time::get_date_with_offset("%Y-%m-%d", -(day as i32));
    let end_date = time::get_date_with_offset("%Y-%m-%d", 2);
    let url = format!(
        "{}?beginDate={}&endDate={}",
        def::RESV_INFO_URL,
        begin_date,
        end_date
    );
    handle_status(def::CLIENT.get(url).send().await?.json::<Resp>().await?)
}

pub async fn cancel(uuid: String) -> Result<Resp> {
    let mut body = HashMap::new();
    body.insert("uuid", uuid.as_str());

    Ok(def::CLIENT
        .post(def::CANCEL_URL)
        .json(&body)
        .send()
        .await?
        .json::<Resp>()
        .await?)
}

pub async fn reserve(
    sites: Option<Vec<String>>,
    filter: Option<Vec<String>>,
    day: u32,
    start: String,
    end: String,
    retry: u32,
) -> Result<Resp> {
    let start = format!("{}:00", start);
    let end = format!("{}:00", end);
    let data = config::load_config_from_file()?.data.clone().unwrap();
    let config = match &data[0] {
        Data::Config(config) => config,
        _ => panic!("load config error"),
    };
    let user = config.user.clone().unwrap();
    let appacc_no = user.accno.parse::<u32>()?;
    let filter: Vec<u32> = handle_filter(filter)?
        .into_iter()
        .map(|x| x.room_id)
        .collect();

    let sites = match sites {
        Some(sites) => {
            let mut ret = Vec::new();
            for site in sites.iter() {
                ret.push(site_name_to_id(site.clone())?);
            }
            ret
        }
        None => {
            let mut ret = Vec::new();
            for _ in 0..=retry {
                ret.push(get_random_site_id(filter.clone())?);
            }
            ret
        }
    };

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.dim.bold} {prefix:.bold.dim} query floor: {wide_msg}",
        )
        .context("Failed to set progress bar style.")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    for &site in sites.iter() {
        pb.set_prefix(format!("[{}]", site_id_to_name(site)?));
        // filter by floor
        let resp = handle_reserve(site, appacc_no, day, start.clone(), end.clone()).await;
        match resp {
            Ok(ret) => {
                if ret.code == 0 {
                    pb.finish_and_clear();
                    return Ok(ret);
                } else {
                    pb.set_message(ret.message);
                    continue;
                }
            }
            Err(_) => continue,
        }
    }
    pb.finish_and_clear();
    Err(anyhow!(RespError::Nodata))
}

async fn handle_reserve(
    site_id: u32,
    appacc_no: u32,
    day: u32,
    start: String,
    end: String,
) -> Result<Resp> {
    let date = time::get_date_with_offset("%Y-%m-%d", day as i32);
    let start_time = format!("{} {}", date, start);
    let end_time = format!("{} {}", date, end);

    let data = serde_json::json!({
        "sysKind": 8,
        "appAccNo": appacc_no,
        "resvMember": [ appacc_no ],
        "resvBeginTime": start_time,
        "resvEndTime": end_time,
        "resvDev": [ site_id ],
    });

    Ok(def::CLIENT
        .post(def::RESERVE_URL)
        .json(&data)
        .send()
        .await?
        .json::<Resp>()
        .await?)
}
