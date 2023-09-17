use super::sign_rec::SignRec;
use super::site::Site;
use super::status::Status;
use crate::utils::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Data {
    Status(Status),
    Site(Site),
    SignRec(SignRec),
    Config(Config),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: u32,
    pub message: String,
    pub data: Option<Vec<Data>>,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Status(resv) => write!(f, "{}", resv),
            Data::Site(site) => write!(f, "{}", site),
            Data::SignRec(sign_rec) => write!(f, "{}", sign_rec),
            Data::Config(config) => write!(f, "{}", config),
        }
    }
}

impl std::fmt::Display for Resp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        if let Some(datas) = self.data.clone() {
            for data in datas {
                write!(f, "{}", data)?;
            }
        }
        Ok(())
    }
}

impl Resp {
    pub fn new(code: u32, message: String, data: Option<Vec<Data>>) -> Self {
        Resp {
            code,
            message,
            data,
        }
    }
}
