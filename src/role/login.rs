use home::home_dir;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

/// Info struct is used to store the information of the user's state.
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

impl Login {
    /// create a new Info struct
    pub fn new(username: String, password: String) -> Self {
        Login { username, password }
    }
    /// save the user's state to the file
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = home_dir().unwrap();
        let path = root.join(".njfu-library-cli.json");

        let mut output = File::create(path)?;
        let info = serde_json::to_string(&self)?;
        write!(output, "{}", info)?;
        Ok(())
    }

    /// # read_from_file
    /// read the user's state from the file
    pub fn read_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let root = home_dir().unwrap();
        let path = root.join(".njfu-library-cli.json");

        let input = File::open(path)?;
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
