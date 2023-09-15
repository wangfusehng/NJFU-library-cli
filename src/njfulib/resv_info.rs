use super::site;
use crate::utils::status;
use crate::utils::time;
use anyhow::Context;
use serde::{Deserialize, Serialize};

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
        writeln!(
            f,
            "dev: {}",
            site::site_id_to_name(self.dev_id)
                .context("invalid dev_id")
                .unwrap()
        )?;
        writeln!(f, "resvName: {}", self.resv_name)?;
        writeln!(
            f,
            "startTime: {}",
            time::get_date_with_time_stamp(self.start_time / 1000)
        )?;
        writeln!(
            f,
            "endTime: {}",
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

impl ResvInfo {
    pub async fn new(
        resv_id: u32,
        resv_name: String,
        start_time: u64,
        end_time: u64,
        resv_status: u32,
        dev_id: u32,
    ) -> Self {
        ResvInfo {
            resv_id,
            resv_name,
            start_time,
            end_time,
            resv_status,
            dev_id,
        }
    }
}
