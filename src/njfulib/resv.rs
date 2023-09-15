use crate::utils::*;
use serde::{Deserialize, Serialize};

use super::site;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SiteInfo {
    #[serde(rename = "resvId")]
    pub resv_id: u32,
    #[serde(rename = "devId")]
    pub dev_id: u32,
}

impl std::fmt::Display for SiteInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", site::site_id_to_name(self.dev_id).unwrap())?;
        Ok(())
    }
}

/// Status struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resv {
    pub uuid: String,
    #[serde(rename = "resvId")]
    pub resv_id: u32,
    #[serde(rename = "appAccNo")]
    pub appacc_no: u32,
    #[serde(rename = "resvBeginTime")]
    pub resv_begin_time: u64,
    #[serde(rename = "resvEndTime")]
    pub resv_end_time: u64,
    #[serde(rename = "resvStatus")]
    pub resv_status: u32,
    #[serde(rename = "resvDevInfoList")]
    pub resv_dev_info_list: Option<Vec<SiteInfo>>,
}

impl std::fmt::Display for Resv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(devs) = &self.resv_dev_info_list {
            for dev in devs {
                write!(f, "{}", dev)?;
            }
        }
        write!(
            f,
            "  {}",
            status::get_color_str_from_resv_status(self.resv_status)
        )?;
        write!(
            f,
            "   {}",
            time::get_date_with_time_stamp(self.resv_begin_time / 1000)
        )?;
        write!(
            f,
            "  {}",
            time::get_date_with_time_stamp(self.resv_end_time / 1000)
        )?;
        writeln!(f, "  {}", self.uuid)?;
        Ok(())
    }
}
