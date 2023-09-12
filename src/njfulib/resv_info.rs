use super::site;
use crate::utils::account;
use crate::utils::time;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ResvInfo {
    #[serde(rename = "resvId")]
    resv_id: u32,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: u64,
    #[serde(rename = "resvStatus")]
    resv_status: u32,
    #[serde(rename = "devId")]
    dev_id: u32,
}

impl std::fmt::Display for ResvInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "resvName: {}",
            account::get_name_by_resv_id(self.resv_id).unwrap()
        )?;
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
        writeln!(f, "resvStatus: {}", self.resv_status)?;
        writeln!(
            f,
            "dev_name: {}",
            site::id_to_name(self.dev_id)
                .context("invalid dev_id")
                .unwrap()
        )?;
        Ok(())
    }
}

impl ResvInfo {
    pub fn new(
        resv_id: u32,
        start_time: u64,
        end_time: u64,
        resv_status: u32,
        dev_id: u32,
    ) -> Self {
        ResvInfo {
            resv_id,
            start_time,
            end_time,
            resv_status,
            dev_id,
        }
    }

    pub fn resv_id(&self) -> u32 {
        self.resv_id
    }

    pub fn start_time(&self) -> u64 {
        self.start_time
    }

    pub fn end_time(&self) -> u64 {
        self.end_time
    }

    pub fn resv_status(&self) -> u32 {
        self.resv_status
    }

    pub fn dev_id(&self) -> u32 {
        self.dev_id
    }
}
