use crate::cli::day::Day;
use crate::cli::reserve;
use crate::role::login::Login;
use crate::role::site;
use crate::role::site::*;
use crate::role::state::State;
use crate::role::student::Student;
use crate::role::ts::Ts;
use crate::utils::html::parse_in;
use crate::utils::*;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Duration;
use std::{cmp::min, fmt::Write};

lazy_static! {
    pub static ref CLIENT: reqwest::blocking::Client = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36"
                    .parse()
                    .context("Failed to parse user agent.").unwrap()
            );
        headers.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("private"),
        );

        reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .expect("Failed to build client.")
    };
}

/// Query the information of a student.
pub fn query_by_name(day: Day, name: String) -> Result<Vec<Site>> {
    let mut body = HashMap::new();
    body.insert("act", "get_rsv_sta");
    let date = match day {
        Day::Today => time::get_date_with_offset("%Y-%m-%d", 0),
        Day::Tomorrow => time::get_date_with_offset("%Y-%m-%d", 1),
        Day::Overmorrow => time::get_date_with_offset("%Y-%m-%d", 2),
    };
    body.insert("date", date.as_str());

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
    let mut ret: Vec<Site> = Vec::new();

    for (pos, (floor_name, floor)) in def::ROOMS.iter().enumerate() {
        pb.set_prefix(format!("[{}/{}]", pos, total));
        pb.set_message(floor_name.to_string());

        let room_id = floor.room_id().to_string();
        let mut data = body.clone();
        data.insert("room_id", room_id.as_str());

        let resp = CLIENT.post(def::DEVICE_URL).form(&data).send()?;

        resp::get_name_info(resp, name.clone()).map(|info| {
            ret.append(info.into_iter().collect::<Vec<Site>>().as_mut());
        })?;
    }

    pb.finish_and_clear();
    Ok(ret)
}

/// Query the information of a site.
pub fn query_by_site(day: Day, site: String) -> Result<Site> {
    let dev_id = name_to_id(site);
    dev_id
        .map(|dev_id| {
            let mut body = HashMap::new();
            let dev_id_binding = dev_id.to_string();
            body.insert("dev_id", dev_id_binding.as_str());
            body.insert("act", "get_rsv_sta");
            let date = match day {
                Day::Today => time::get_date_with_offset("%Y-%m-%d", 0),
                Day::Tomorrow => time::get_date_with_offset("%Y-%m-%d", 1),
                Day::Overmorrow => time::get_date_with_offset("%Y-%m-%d", 2),
            };
            body.insert("date", date.as_str());

            let resp = CLIENT.post(def::DEVICE_URL).form(&body).send()?;

            resp::get_site_info(resp)
        })
        .context("query by site fail")?
}

/// handle actual login to the server.
fn handle_login() -> Result<Student> {
    let mut login = Login::default();
    login.read_from_file().context("read student info failed")?;

    let mut body = HashMap::new();
    body.insert("act", "login");
    body.insert("id", login.username());
    body.insert("pwd", login.password());

    let resp = CLIENT.post(def::LOGIN_URL).form(&body).send()?;

    resp::get_login_info(resp)
}

/// login to the server.
pub fn login(username: String, password: String) -> Result<Student> {
    let student = Login::new(username, password);
    student.save_to_file().context("save student info failed")?;
    handle_login()
}

/// query the user status.
pub fn state() -> Result<Vec<State>> {
    //login
    handle_login()?;

    let mut body = HashMap::new();
    body.insert("act", "get_History_resv");
    body.insert("strat", "90");
    body.insert("StatFlag", "New");

    let resp = CLIENT.post(def::CENTER_URL).form(&body).send()?;

    resp::get_state_info(resp)
}

/// cancel the reservation.
pub fn cancel(id: String) -> Result<String> {
    //login
    handle_login()?;

    let mut body = HashMap::new();
    body.insert("act", "del_resv");
    body.insert("id", id.as_str());

    let resp = CLIENT.post(def::RESERVE_URL).form(&body).send()?;

    resp::get_cancel_info(resp)
}

/// use card id to get user id in library
fn search_account(card_id: String) -> Result<String> {
    let ret = CLIENT
        .post(def::SEARCHACCOUNT_URL)
        .form(&[("term", card_id)])
        .send()?;
    resp::get_account_info(ret)
}

fn handle_reserve(
    site: String,
    user: Option<Vec<String>>,
    day: Day,
    start: String,
    end: String,
) -> Result<String> {
    let id = name_to_id(site)?;

    let mut body = HashMap::new();
    body.insert("act", "set_resv");
    let id_binding = id.to_string();
    body.insert("dev_id", id_binding.as_str());

    let day = match day {
        Day::Today => time::get_date_with_offset("%Y-%m-%d", 0),
        Day::Tomorrow => time::get_date_with_offset("%Y-%m-%d", 1),
        Day::Overmorrow => time::get_date_with_offset("%Y-%m-%d", 2),
    };
    let start_time = format!("{} {}", day, start);
    let end_time = format!("{} {}", day, end);
    body.insert("start", start_time.as_str());
    body.insert("end", end_time.as_str());

    let mut user_list = String::from("$");

    if let Some(user) = user {
        let users = user
            .iter()
            .map(|x| search_account(x.to_string()).expect("search account error"))
            .collect::<Vec<String>>()
            .join(",");

        user_list.push_str(users.as_str());

        // if have -u or --user
        body.insert("min_user", "0");
        body.insert("max_user", "8");
        body.insert("mb_list", user_list.as_str());
    }
    let resp = CLIENT.post(def::RESERVE_URL).form(&body).send()?;
    resp::get_reserve_info(resp)
}

/// reserve the site.
pub fn reserve(
    sites: Option<Vec<String>>,
    filter: Vec<String>,
    user: Option<Vec<String>>,
    day: Day,
    start: String,
    end: String,
    retry: u32,
) -> Result<String> {
    //login
    handle_login()?;

    match sites {
        Some(sites) => {
            for site in sites {
                if split_site(site.clone()).is_ok() {
                    // filter by floor
                    if !(site_fiter_by_floor(site.clone(), filter.clone())?) {
                        continue;
                    }
                }
                let resp = handle_reserve(
                    site.clone(),
                    user.clone(),
                    day.clone(),
                    start.clone(),
                    end.clone(),
                );
                match resp {
                    Ok(resp) => {
                        println!("{}: {}\n", site, resp);
                        if resp.contains("成功") {
                            return Ok(resp);
                        }
                    }
                    // the current site can not be reserved
                    // test next site
                    Err(e) => println!("{}", e),
                }
            }
            Err(anyhow!("no site from put in can be reserved"))
        }
        None => {
            let mut cnt = 0;
            while cnt < retry {
                let site = site::get_random_site_name()?;

                // filter by floor
                if !(site_fiter_by_floor(site.clone(), filter.clone())?) {
                    continue;
                }
                let resp = handle_reserve(
                    site.clone(),
                    user.clone(),
                    day.clone(),
                    start.clone(),
                    end.clone(),
                );
                match resp {
                    Ok(resp) => {
                        println!("{}: {}\n", site, resp);
                        if resp.contains("成功") {
                            return Ok(resp);
                        }
                    }
                    Err(e) => return Err(e),
                }
                cnt += 1;
            }
            Err(anyhow!("time out for reserve random site"))
        }
    }
}

///check in reserve on time
pub fn check_in(site: String, time: Option<u32>) -> Result<String> {
    // get site info
    let site = query_by_site(Day::Today, site)?;
    // get stduent info
    let student = handle_login()?;

    let mut body = HashMap::new();
    body.insert("lab", site.lab_id());
    body.insert("dev", site.dev_id());
    body.insert("msn", student.msn());

    let content_lenth = reqwest::header::HeaderValue::from_str("0")?;

    let resp = CLIENT
        .post(def::WXSEATSIGN)
        .header(reqwest::header::CONTENT_LENGTH, content_lenth)
        .form(&body)
        .send()?
        .text()?;
    // get dafault left time
    let opt = html::parse_site_login(resp)?;

    let mut body = HashMap::new();
    body.insert("DoUserIn", "true");
    // default use opt
    let time_binding = time.unwrap_or(opt).to_string();
    body.insert("dwUseMin", time_binding.as_str());

    let resp = CLIENT.post(def::WXSEATSIGN).form(&body).send()?.text()?;

    parse_in(resp)
}

/// check out site
pub fn check_out(id: String) -> Result<String> {
    //login
    handle_login()?;

    let mut body = HashMap::new();
    body.insert("act", "resv_leave");
    body.insert("type", "2");
    body.insert("resv_id", id.as_str());

    let resp = CLIENT.post(def::RESERVE_URL).form(&body).send()?;

    resp::get_check_out_info(resp)
}
