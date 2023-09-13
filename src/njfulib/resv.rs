use super::dev::Dev;
use crate::utils::status;
use crate::utils::time;
use serde::{Deserialize, Serialize};

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
    pub resv_dev_info_list: Option<Vec<Dev>>,
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

impl Resv {
    pub fn new(
        uuid: String,
        resv_id: u32,
        appacc_no: u32,
        resv_begin_time: u64,
        resv_end_time: u64,
        resv_status: u32,
        resv_dev_info_list: Option<Vec<Dev>>,
    ) -> Self {
        Resv {
            uuid,
            resv_id,
            appacc_no,
            resv_begin_time,
            resv_end_time,
            resv_status,
            resv_dev_info_list,
        }
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn resv_id(&self) -> u32 {
        self.resv_id
    }

    pub fn appacc_no(&self) -> u32 {
        self.appacc_no
    }

    pub fn resv_begin_time(&self) -> u64 {
        self.resv_begin_time
    }

    pub fn resv_end_time(&self) -> u64 {
        self.resv_end_time
    }

    pub fn resv_status(&self) -> u32 {
        self.resv_status
    }

    pub fn resv_dev_info_list(&self) -> Option<&Vec<Dev>> {
        self.resv_dev_info_list.as_ref()
    }
}
