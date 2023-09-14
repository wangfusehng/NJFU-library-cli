use crate::njfulib::site;
use serde::{Deserialize, Serialize};

use crate::def;

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

impl std::fmt::Display for SignRec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, "uuid: {}", self.uuid)?;
        writeln!(f, "resv_id: {}", self.resv_id)?;
        writeln!(f, "card: {}", self.logon_name)?;
        writeln!(f, "name: {}", self.true_name)?;
        writeln!(f, "dev: {}", site::site_id_to_name(self.dev_id).unwrap())?;
        Ok(())
    }
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
