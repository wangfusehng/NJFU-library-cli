use super::resv::Resv;
use super::site::Site;
use super::{dev::Dev, sign_rec::SignRec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Data {
    Resv(Resv),
    Site(Site),
    SignRec(SignRec),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    message: String,
    data: Option<Vec<Data>>,
}

impl Resp {
    pub fn new(message: String, data: Option<Vec<Data>>) -> Self {
        Resp { message, data }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn data(&self) -> &Option<Vec<Data>> {
        &self.data
    }
}
