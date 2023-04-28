use crate::role::floor::Floor;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref FLOOR: Vec<&'static str> = vec![
            "2F-A", "2F-B", "3F-A", "3F-B", "3F-C", "3FA-", "4F-A", "4FA-", "5F-A", "6F-A", "7F-A",
        ];
    /// line separator
    pub static ref LINE_SEPARATOR: String = String::from("--------------------\n");
    ///Library base url
    pub static ref BASE_URL: String = String::from("https://libic.njfu.edu.cn/ClientWeb/pro/ajax/");
    ///Library device url
    pub static ref DEVICE_URL: String = BASE_URL.to_owned() + "device.aspx";
    ///Library login url
    pub static ref LOGIN_URL: String = BASE_URL.to_owned() + "login.aspx";
    ///Library center url
    pub static ref CENTER_URL: String = BASE_URL.to_owned() + "center.aspx";
    /// Library reserve url
    pub static ref RESERVE_URL: String = BASE_URL.to_owned() + "reserve.aspx";
    /// Library room info
    pub static ref ROOMS: HashMap<&'static str, Floor> = {
        let mut map = HashMap::new();
        map.insert("2F-A", Floor::new(100455344, 100455361, 100455800, 440));
        map.insert("2F-B", Floor::new(100455346, 100455802, 100455897, 96));
        map.insert("3F-A", Floor::new(100455350, 100456256, 100456658, 403));
        map.insert("3F-B", Floor::new(100455352, 100456660, 100456791, 132));
        map.insert("3F-C", Floor::new(100455354, 100499567, 100499728, 162));
        map.insert("3FA-", Floor::new(111488386, 111488493, 111488512, 020));
        map.insert("4F-A", Floor::new(100455356, 100499729, 100500156, 428));
        map.insert("4FA-", Floor::new(111488388, 111488513, 111488536, 024));
        map.insert("5F-A", Floor::new(100455358, 100500173, 100500532, 360));
        map.insert("6F-A", Floor::new(100455360, 100500602, 100500949, 348));
        map.insert("7F-A", Floor::new(106658017, 106744855, 106745104, 224));
        map
    };

    /// rewest header
    pub static ref HEADERMAP: reqwest::header::HeaderMap = {
        let mut headermap = reqwest::header::HeaderMap::new();
        headermap.insert(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36"
                .parse()
                .unwrap(),
        );
        headermap.insert(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headermap.insert(
            reqwest::header::CACHE_CONTROL,
            reqwest::header::HeaderValue::from_static("private"),
        );
        headermap
    };
    pub static ref CLIENT: reqwest::blocking::Client = {
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();

        client
    };
}
