use super::dev::Dev;
use serde::{Deserialize, Serialize};

/// State struct
/// State struct is used to store the information of the user's state.
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
    #[serde(rename = "resvDevInfoList")]
    pub resv_dev_info_list: Option<Vec<Dev>>,
}

impl std::fmt::Display for Resv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.uuid, self.resv_begin_time, self.resv_end_time
        )
    }
}

impl Resv {
    pub fn new(
        uuid: String,
        resv_id: u32,
        appacc_no: u32,
        resv_begin_time: u64,
        resv_end_time: u64,
        resv_dev_info_list: Option<Vec<Dev>>,
    ) -> Self {
        Resv {
            uuid,
            resv_id,
            appacc_no,
            resv_begin_time,
            resv_end_time,
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

    pub fn resv_dev_info_list(&self) -> Option<&Vec<Dev>> {
        self.resv_dev_info_list.as_ref()
    }
}
