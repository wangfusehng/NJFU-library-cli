use crate::njfulib::config;
use crate::njfulib::floor::Floor;
use crate::njfulib::resp::Data;
use anyhow::Context;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const LONG_LINE_SEPARATOR: &str = "--------------------";
pub const SHORT_LINE_SEPARATOR: &str = "---------";
pub const BASE_URL: &str = "https://libseat.njfu.edu.cn/";
// 查询个人信息
pub const USER_INFO_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/login.aspx";
// 个人预约记录
pub const RESV_INFO_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve/resvInfo";
// 取消
pub const CANCEL_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve/delete";
// 预约
pub const RESERVE_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve";
// 查询
pub const QUERY_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve";
// 使用 resvId 查询用户信息
pub const SEARCHACCOUNT_URL: &str = "https://libseat.njfu.edu.cn/ic-web/reserve/getSignRec";

lazy_static! {
    pub static ref FLOOR: Vec<&'static str> = vec![
        "2F-A", "2F-B", "3F-A", "3F-B", "3F-C", "3FA-", "4F-A", "4FA-", "5F-A", "6F-A", "7F-A",
    ];
    pub static ref ROOMS: HashMap<&'static str, Floor> = {
        let mut map = HashMap::new();
        map.insert("2F-A", Floor::new(100455344, 100455361, 100455800, 440));
        map.insert("2F-B", Floor::new(100455346, 100455802, 100455897, 96));
        map.insert("3F-A", Floor::new(100455350, 100456256, 100456658, 403));
        map.insert("3F-B", Floor::new(100455352, 100456660, 100456791, 132));
        map.insert("3F-C", Floor::new(100455354, 100499567, 100499728, 162));
        map.insert("3FA-", Floor::new(111488386, 111488493, 111488512, 20));
        map.insert("4F-A", Floor::new(100455356, 100499729, 100500156, 428));
        map.insert("4FA-", Floor::new(111488388, 111488513, 111488536, 24));
        map.insert("5F-A", Floor::new(100455358, 100500173, 100500532, 360));
        map.insert("6F-A", Floor::new(100455360, 100500602, 100500949, 348));
        map.insert("7F-A", Floor::new(106658017, 106744855, 106745104, 224));
        map
    };
    pub static ref SPACE: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("8A505", 100504089);
        map.insert("8A506", 100504093);
        map.insert("8A508", 100504097);
        map.insert("8A512", 100504101);
        map.insert("8A513", 100504105);
        map.insert("8A515", 100504109);
        map
    };
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

        let data = config::load_config_from_file()
            .unwrap()
            .data()
            .clone()
            .unwrap();

        let config = match &data[0] {
            Data::Config(config) => config,
            _ => panic!("Failed to load config from file."),
        };

        headers.insert(
            reqwest::header::COOKIE,
            format!("ic-cookie={}", config.cookie()).parse().unwrap(),
        );

        reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .expect("Failed to build client.")
    };
    pub static ref OLD_CLIENT: reqwest::blocking::Client = {
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
