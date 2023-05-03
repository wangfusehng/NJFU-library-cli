use super::ts::Ts;
use crate::utils::def;
use anyhow::{anyhow, Context, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    #[serde(rename = "devName")]
    dev_name: String,
    #[serde(rename = "devId")]
    dev_id: String,
    #[serde(rename = "labId")]
    lab_id: String,
    ts: Option<Vec<Ts>>,
}

impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "dev_name: {}", self.dev_name)?;
        writeln!(f, "owner:")?;
        self.ts.as_ref().map(|ts| -> Result<(), std::fmt::Error> {
            for t in ts {
                writeln!(f, "{}", t)?;
                writeln!(f, "{}", def::LINE_SEPARATOR)?;
            }
            Ok(())
        });
        Ok(())
    }
}

impl Site {
    pub fn new(dev_name: String, dev_id: String, lab_id: String, ts: Option<Vec<Ts>>) -> Self {
        Site {
            dev_name,
            dev_id,
            lab_id,
            ts,
        }
    }

    pub fn set_dev_name(&mut self, dev_name: String) {
        self.dev_name = dev_name;
    }

    pub fn dev_name(&self) -> &str {
        self.dev_name.as_ref()
    }

    pub fn dev_id(&self) -> &str {
        self.dev_id.as_ref()
    }

    pub fn set_dev_id(&mut self, dev_id: String) {
        self.dev_id = dev_id;
    }

    pub fn ts(&self) -> Option<&Vec<Ts>> {
        self.ts.as_ref()
    }

    pub fn set_ts(&mut self, ts: Option<Vec<Ts>>) {
        self.ts = ts;
    }

    pub fn lab_id(&self) -> &str {
        self.lab_id.as_ref()
    }

    pub fn set_lab_id(&mut self, lab_id: String) {
        self.lab_id = lab_id;
    }
}

/// tranform the site to the site id
pub fn site_name_to_id(site: String) -> Result<u32> {
    let floor = &site[0..4];
    match &site[4..].parse() {
        Ok(_site) => {
            let floor = def::ROOMS.get(floor);
            match floor {
                Some(floor) => {
                    let id = floor.dev_start() + _site - 1;

                    if id >= floor.dev_start() && id <= floor.dev_end() {
                        Ok(id)
                    } else {
                        Err(anyhow!("parse room id error"))
                    }
                }
                None => Err(anyhow!("parse room id error")),
            }
        }
        Err(e) => Err(anyhow!("parse room id error: {}", e)),
    }
}

/// tranform site id to site name
pub fn site_id_to_name(id: u32) -> Result<String> {
    let mut floor = "";
    for (k, v) in def::ROOMS.iter() {
        if id >= v.dev_start() && id <= v.dev_end() {
            floor = k;
            break;
        }
    }
    if floor.is_empty() {
        return Err(anyhow!("parse id to name error"));
    }
    let site = id - def::ROOMS.get(floor).unwrap().dev_start() + 1;
    Ok(format!("{}{:03}", floor, site))
}

/// site id fiter by floor
pub fn site_id_fiter_by_floor(id: u32, floor: Vec<String>) -> Result<bool> {
    let mut floor_name = "";
    for i in floor.iter() {
        let v = def::ROOMS
            .get(i.as_str())
            .ok_or(anyhow!("parse room error"))?;

        if id >= v.dev_start() && id <= v.dev_end() {
            floor_name = i.as_str();
            break;
        }
    }
    if floor_name.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

/// get random site name in libray
pub fn get_random_site_name() -> Result<String> {
    let mut rng = rand::thread_rng();
    let floor_name = def::FLOOR
        .get(rng.gen_range(0..def::FLOOR.len()))
        .context("get floor name error")?;
    let floor = def::ROOMS.get(floor_name).context("get floor error")?;
    let site_id = rng.gen_range(floor.dev_start()..floor.dev_end() + 1);
    site_id_to_name(site_id)
}

#[test]
fn test_site() {
    let site = "5F-A100".to_string();
    let get_site = site_id_to_name(site_name_to_id(site.to_string()).unwrap()).unwrap();
    assert_eq!(site, get_site);
}
