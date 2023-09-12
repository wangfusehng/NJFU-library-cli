use super::config::Config;
use super::resv::Resv;
use super::sign_rec::SignRec;
use super::site::Site;
use crate::def;
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

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Resv(resv) => write!(f, "{}", resv),
            Data::Site(site) => write!(f, "{}", site),
            Data::SignRec(sign_rec) => write!(f, "{}", sign_rec),
            Data::Config(config) => write!(f, "{}", config),
        }
    }
}

impl std::fmt::Display for Resp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, "message: {}", self.message)?;
        if let Some(datas) = self.data() {
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
