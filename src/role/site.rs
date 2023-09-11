use super::dev::Dev;
use super::floor::Floor;
use crate::def;
use anyhow::{anyhow, Context, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    #[serde(rename = "devId")]
    dev_id: u32,
    coordinate: String,
    #[serde(rename = "labId")]
    lab_id: u32,
    #[serde(rename = "resvInfo", default)]
    resv_info: Option<Vec<Dev>>,
}

impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-----")?;
        self.resv_info
            .as_ref()
            .map(|ts| -> Result<(), std::fmt::Error> {
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
    pub fn new(dev_id: u32, coordinate: String, lab_id: u32, resv_info: Option<Vec<Dev>>) -> Self {
        Site {
            dev_id,
            coordinate,
            lab_id,
            resv_info,
        }
    }

    pub fn dev_id(&self) -> u32 {
        self.dev_id
    }

    pub fn coordinate(&self) -> &str {
        &self.coordinate
    }

    pub fn lab_id(&self) -> u32 {
        self.lab_id
    }

    pub fn resv_info(&self) -> Option<&Vec<Dev>> {
        self.resv_info.as_ref()
    }
}

pub fn name_to_floor(dev_name: String) -> Result<&'static Floor> {
    def::ROOMS
        .get(&dev_name[0..4])
        .context(format!("Failed to get floor for {}", &dev_name[0..4]))
}

pub fn id_to_name(id: u32) -> Result<String> {
    // search space
    for (k, v) in def::SPACE.iter() {
        if id == *v {
            return Ok(k.to_string());
        }
    }
    // search site
    let mut floor = "";
    for (k, v) in def::ROOMS.iter() {
        if id >= v.dev_start() && id <= v.dev_end() {
            floor = k;
            break;
        }
    }
    if floor.is_empty() {
        anyhow::bail!("{} is not a site or space", id);
    }
    let site = id
        - def::ROOMS
            .get(floor)
            .context(format!("Failed to get floor {}", floor))?
            .dev_start()
        + 1;
    Ok(format!("{}{:03}", floor, site))
}

pub fn name_to_id(dev_name: String) -> Result<u32> {
    let floor = name_to_floor(dev_name.clone())?;
    let site = &dev_name[4..].parse()?;
    let id = floor.dev_start() + site - 1;
    if id >= floor.dev_start() && id <= floor.dev_end() {
        Ok(id)
    } else {
        anyhow::bail!("{} is not a site", dev_name)
    }
}

pub fn site_fiter_by_floor(site: String, floor: Vec<String>) -> Result<bool> {
    let id = name_to_id(site)?;
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
    id_to_name(site_id)
}
