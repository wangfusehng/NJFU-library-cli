use std::fmt::write;

use serde::{Deserialize, Serialize};

/// # Student struct
/// Student struct is used to store student's information
/// The information includes the name and the id of the student
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    name: String,
    id: String,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}\nid: {}", self.name, self.id)
    }
}

impl Student {
    pub fn new(name: String, id: String) -> Self {
        Student { name, id }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}
