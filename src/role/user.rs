use super::config::Config;
use crate::def;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: String,
    accno: String,
    name: String,
    phone: String,
    email: String,
    dept: String,
}

impl User {
    pub fn new() -> Self {
        User {
            id: String::new(),
            accno: String::new(),
            name: String::new(),
            phone: String::new(),
            email: String::new(),
            dept: String::new(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn accno(&self) -> &String {
        &self.accno
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn phone(&self) -> &String {
        &self.phone
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn dept(&self) -> &String {
        &self.dept
    }
}

pub fn search_user_info(config: &Config) -> Result<User> {
    let mut form = HashMap::new();
    form.insert("id", config.username());
    form.insert("pwd", config.password());
    form.insert("act", "login");
    let resp = def::OLD_CLIENT
        .post(def::USER_INFO_URL)
        .form(&form)
        .send()?;
    let resp = resp.json::<serde_json::Value>()?;

    Ok(serde_json::from_value(resp["data"].clone())?)
}
