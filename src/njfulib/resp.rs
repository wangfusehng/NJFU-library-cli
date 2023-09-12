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
        writeln!(f, "{}", self.message())?;
        if let Some(datas) = self.data().clone() {
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

    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn set_data(&mut self, data: Option<Vec<Data>>) {
        self.data = data
    }
}
