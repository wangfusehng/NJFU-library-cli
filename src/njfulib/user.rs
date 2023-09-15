use super::config::Config;
use crate::def;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub accno: String,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub dept: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, "   id: {}", self.id)?;
        writeln!(f, "accno: {}", self.accno)?;
        writeln!(f, " name: {}", self.name)?;
        writeln!(f, "phone: {}", self.phone)?;
        writeln!(f, "email: {}", self.email)?;
        writeln!(f, " dept: {}", self.dept)?;
        Ok(())
    }
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
}

pub fn search_user_info(config: &Config) -> Result<User> {
    let mut form = HashMap::new();
    form.insert("id", config.username.as_str());
    form.insert("pwd", config.password.as_str());
    form.insert("act", "login");
    let resp = def::OLD_CLIENT
        .post(def::USER_INFO_URL)
        .form(&form)
        .send()?;
    let resp = resp.json::<serde_json::Value>()?;

    Ok(serde_json::from_value(resp["data"].clone())?)
}
