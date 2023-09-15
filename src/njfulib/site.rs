use super::floor;
use super::floor::Floor;
use crate::utils::*;
use crate::{def, error::ClientError};
use anyhow::{anyhow, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    #[serde(rename = "devId")]
    pub dev_id: u32,
    pub coordinate: String,
    #[serde(rename = "roomId")]
    pub room_id: u32,
    #[serde(rename = "labId")]
    pub lab_id: u32,
    #[serde(rename = "resvInfo")]
    pub resv_info: Option<Vec<ResvInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResvInfo {
    #[serde(rename = "resvId")]
    pub resv_id: u32,
    #[serde(rename = "resvName", skip)]
    pub resv_name: String,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: u64,
    #[serde(rename = "resvStatus")]
    pub resv_status: u32,
    #[serde(rename = "devId")]
    pub dev_id: u32,
}

impl std::fmt::Display for ResvInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "       dev: {}", site_id_to_name(self.dev_id).unwrap())?;
        writeln!(f, "  resvName: {}", self.resv_name)?;
        writeln!(
            f,
            " startTime: {}",
            time::get_date_with_time_stamp(self.start_time / 1000)
        )?;
        writeln!(
            f,
            "   endTime: {}",
            time::get_date_with_time_stamp(self.end_time / 1000)
        )?;
        writeln!(
            f,
            "resvStatus: {}",
            status::get_color_str_from_resv_status(self.resv_status)
        )?;
        Ok(())
    }
}

impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::SHORT_LINE_SEPARATOR)?;
        self.resv_info
            .as_ref()
            .map(|ts| -> Result<(), std::fmt::Error> {
                for t in ts {
                    write!(f, "{}", t)?;
                }
                Ok(())
            });
        Ok(())
    }
}

pub fn site_name_to_floor(dev_name: String) -> Result<Floor> {
    let floor_name = &dev_name[0..4];
    floor::get_floor_by_room_name(floor_name)
}

pub fn site_id_to_name(site_id: u32) -> Result<String> {
    let floor = site_id_to_floor(site_id)?;
    let site_no = site_id - floor.dev_start + 1;
    Ok(format!("{}{:0>3}", floor.room_name, site_no))
}

pub fn site_id_to_floor(site_id: u32) -> Result<Floor> {
    for floor in def::FLOORS.iter() {
        if site_id >= floor.dev_start && site_id <= floor.dev_end {
            return Ok(floor.clone());
        }
    }
    Err(anyhow!(ClientError::SiteError))
}

pub fn site_name_to_id(dev_name: String) -> Result<u32> {
    let floor = site_name_to_floor(dev_name.clone())?;
    let site = &dev_name[4..].parse()?;
    let id = floor.dev_start + site - 1;
    if id >= floor.dev_start && id <= floor.dev_end {
        Ok(id)
    } else {
        Err(anyhow!(ClientError::SiteError))
    }
}

pub fn site_fiter_by_floor(site: u32, floors: Vec<u32>) -> Result<bool> {
    for i in floors {
        let floor = floor::get_floor_by_room_id(i)?;
        if site >= floor.dev_start && site <= floor.dev_end {
            return Ok(true);
        }
    }
    Ok(false)
}

/// get random site name in libray
pub fn get_random_site_id(filter: Vec<u32>) -> Result<u32> {
    let len = filter.len();
    let mut rng = rand::thread_rng();
    let index = filter.get(rng.gen_range(0..len)).unwrap();
    let floor = floor::get_floor_by_room_id(*index)?;
    Ok(rng.gen_range(floor.dev_start..floor.dev_end + 1))
}
