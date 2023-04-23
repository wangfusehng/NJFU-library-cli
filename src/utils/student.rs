use home::home_dir;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub username: String,
    pub password: String,
}

impl Student {
    pub fn new(username: String, password: String) -> Self {
        Student { username, password }
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = home_dir().unwrap();
        let path = root.join(".njfu-library-cli.json");

        let mut output = File::create(path)?;
        let info = serde_json::to_string(&self)?;
        write!(output, "{}", info)?;
        Ok(())
    }

    pub fn read_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let root = home_dir().unwrap();
        let path = root.join(".njfu-library-cli.json");

        let input = File::open(path)?;
        let student: Student = serde_json::from_reader(input)?;
        self.username = student.username;
        self.password = student.password;

        Ok(())
    }
}
