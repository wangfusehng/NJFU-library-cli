use crate::role::floor::Floor;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const LINE_SEPARATOR: &str = "--------------------";
pub const BASE_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/";
pub const DEVICE_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/device.aspx";
pub const LOGIN_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/login.aspx";
pub const CENTER_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/center.aspx";
pub const RESERVE_URL: &str = "https://libic.njfu.edu.cn/ClientWeb/pro/ajax/reserve.aspx";
pub const WXSEATSIGN: &str = "https://libic.njfu.edu.cn/Pages/WxSeatSign.aspx";
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
}
