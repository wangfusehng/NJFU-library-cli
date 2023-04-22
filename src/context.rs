use crate::utils::*;
use log::*;
use std::collections::HashMap;

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }

    pub fn query_by_name(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        if name.is_none() {
            error!("Please input a name");
            return Ok(());
        }
        let client = reqwest::blocking::Client::new();

        let mut data = HashMap::new();
        data.insert("byType", "devcls");
        data.insert("classkind", "8");
        data.insert("cld_name", "default");
        data.insert("act", "get_rsv_sta");
        let date = time::get_date("%Y-%m-%d");
        data.insert("date", date.as_str());

        for (room_name, room_id) in def::ROOMS.iter() {
            debug!("querying room {}", room_name);
            let mut data = data.clone();
            let room_id = room_id.0.to_string();
            data.insert("room_id", room_id.as_str());

            let resp = client
                .post(def::DEVICE_URL.as_str())
                .headers(def::HEADERS.clone())
                .form(&data)
                .send()?
                .json::<serde_json::Value>()?;

            let p = resp["data"].as_array().unwrap();
            for i in p {
                if i["ts"][0]["owner"] == name.clone().unwrap().as_str() {
                    println!(
                        "{} {} start:{} end:{}",
                        i["name"], i["ts"][0]["owner"], i["ts"][0]["start"], i["ts"][0]["end"]
                    );
                }
            }
        }
        Ok(())
    }

    pub fn query_by_site(&self, site: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        if site.is_none() {
            error!("please input site");
            return Ok(());
        }
        let dev_id = self.get_site_id(site.unwrap()).to_string();

        let client = reqwest::blocking::Client::new();

        let mut data = HashMap::new();
        data.insert("dev_id", dev_id.as_str());
        data.insert("act", "get_rsv_sta");
        let date = time::get_date("%Y-%m-%d");
        data.insert("date", date.as_str());

        let resp = client
            .post(def::DEVICE_URL.as_str())
            .headers(def::HEADERS.clone())
            .form(&data)
            .send()?
            .json::<serde_json::Value>()?;

        let p = resp["data"].as_array().unwrap();
        let site = p[0].to_owned();
        println!("site:{}", site["name"]);
        let ts = site["ts"].as_array().unwrap();
        if ts.len() != 0 {
            for i in ts {
                println!("owner:{} start:{} end:{}", i["owner"], i["start"], i["end"]);
            }
        } else {
            println!("no owner");
        }
        Ok(())
    }

    pub fn get_site_id(&self, site: String) -> u32 {
        let _floor = &site[0..4];
        match &site[4..].parse() {
            Ok(_site) => {
                let floor = def::ROOMS.get(_floor);
                match floor {
                    Some(floor) => floor.1 + _site - 1,
                    None => {
                        panic!("no such site");
                    }
                }
            }
            Err(e) => {
                panic!("parse room id error: {}", e);
            }
        }
    }
}
