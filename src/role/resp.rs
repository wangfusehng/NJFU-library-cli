use super::config::Config;
use super::resv::Resv;
use super::sign_rec::SignRec;
use super::site::Site;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Data {
    Resv(Resv),
    Site(Site),
    SignRec(SignRec),
    Config(Config),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    code: u32,
    message: String,
    data: Option<Vec<Data>>,
}

impl Resp {
    pub fn new(code: u32, message: String, data: Option<Vec<Data>>) -> Self {
        Resp {
            code,
            message,
            data,
        }
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn data(&self) -> &Option<Vec<Data>> {
        &self.data
    }
}
