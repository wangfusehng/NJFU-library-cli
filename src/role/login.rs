use anyhow::{Context, Result};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// Info struct is used to store the information of the user's state.
#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

impl std::fmt::Display for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "username: {}\npassword: *********", self.username)
    }
}

impl Login {
    /// create a new Info struct
    pub fn new(username: String, password: String) -> Self {
        Login { username, password }
    }
    pub fn is_existed(&self) -> Result<bool> {
        let root = home_dir().context("can not get home dir")?;
        let path = root.join(".njfu-library-cli.json");
        Ok(path.exists())
    }

    /// save the user's state to the file
    pub fn save_to_file(&self) -> Result<()> {
        let root = home_dir().context("can not get home dir")?;
        let path = root.join(".njfu-library-cli.json");

        let mut output = File::create(path)?;
        let info = serde_json::to_string(&self)?;
        write!(output, "{}", info)?;
        Ok(())
    }

    /// # read_from_file
    /// read the user's state from the file
    pub fn read_from_file(&mut self) -> Result<()> {
        let root = home_dir().context("can not get home dir")?;
        let path = root.join(".njfu-library-cli.json");

        let input = File::open(path).context("please login first")?;
        let student: Login = serde_json::from_reader(input)?;
        self.username = student.username;
        self.password = student.password;

        Ok(())
    }

    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
