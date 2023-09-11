use super::dev::Dev;
use super::resv::Resv;
use super::site::Site;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRec {
    uuid: String,
    sid: u32,
    #[serde(rename = "resvId")]
    resv_id: u32,
    #[serde(rename = "accNo")]
    acc_no: u32,
    #[serde(rename = "logonName")]
    logon_name: String,
    #[serde(rename = "trueName")]
    true_name: String,
    #[serde(rename = "devId")]
    dev_id: u32,
    #[serde(rename = "roomId")]
    room_id: u32,
    #[serde(rename = "labId")]
    lab_id: u32,
    #[serde(rename = "createTime")]
    create_time: u64,
}

impl SignRec {
    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn sid(&self) -> u32 {
        self.sid
    }

    pub fn resv_id(&self) -> u32 {
        self.resv_id
    }

    pub fn acc_no(&self) -> u32 {
        self.acc_no
    }

    pub fn logon_name(&self) -> &str {
        &self.logon_name
    }

    pub fn true_name(&self) -> &str {
        &self.true_name
    }

    pub fn dev_id(&self) -> u32 {
        self.dev_id
    }

    pub fn room_id(&self) -> u32 {
        self.room_id
    }

    pub fn lab_id(&self) -> u32 {
        self.lab_id
    }

    pub fn create_time(&self) -> u64 {
        self.create_time
    }
}
