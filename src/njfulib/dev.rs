use super::site;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Dev {
    #[serde(rename = "resvId")]
    resv_id: u32,
    #[serde(rename = "devId")]
    dev_id: u32,
}

impl std::fmt::Display for Dev {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            site::site_id_to_name(self.dev_id)
                .context("invalid dev_id")
                .unwrap()
        )?;
        Ok(())
    }
}

impl Dev {
    pub fn new(resv_id: u32, dev_id: u32) -> Self {
        Dev { resv_id, dev_id }
    }

    pub fn resv_id(&self) -> u32 {
        self.resv_id
    }

    pub fn dev_id(&self) -> u32 {
        self.dev_id
    }
}
