use crate::njfulib::site;
use serde::{Deserialize, Serialize};

use crate::def;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRec {
    pub uuid: String,
    pub sid: u32,
    #[serde(rename = "resvId")]
    pub resv_id: u32,
    #[serde(rename = "accNo")]
    pub acc_no: u32,
    #[serde(rename = "logonName")]
    pub logon_name: String,
    #[serde(rename = "trueName")]
    pub true_name: String,
    #[serde(rename = "devId")]
    pub dev_id: u32,
    #[serde(rename = "roomId")]
    pub room_id: u32,
    #[serde(rename = "labId")]
    pub lab_id: u32,
    #[serde(rename = "createTime")]
    pub create_time: u64,
}

impl std::fmt::Display for SignRec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, " dev: {}", site::site_id_to_name(self.dev_id).unwrap())?;
        writeln!(f, "name: {}", self.true_name)?;
        writeln!(f, "card: {}", self.logon_name)?;
        Ok(())
    }
}
