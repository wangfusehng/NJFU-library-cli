use serde::{Deserialize, Serialize};

/// # Student struct
/// Student struct is used to store student's information
/// The information includes the name and the id of the student
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    name: String,
    id: String,
    msn: String,
    dept: String,
    credit: Vec<Vec<String>>,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let credit = format!("{}/{}", &self.credit[0][1], &self.credit[0][2]);
        write!(
            f,
            "name:\t{}\nid:\t{}\ndept:\t{}\ncredit:\t{}",
            self.name, self.id, self.dept, credit
        )
    }
}

impl Student {
    pub fn new(
        name: String,
        id: String,
        msn: String,
        dept: String,
        credit: Vec<Vec<String>>,
    ) -> Self {
        Student {
            name,
            id,
            msn,
            dept,
            credit,
        }
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

    pub fn dept(&self) -> &str {
        self.dept.as_ref()
    }

    pub fn set_dept(&mut self, dept: String) {
        self.dept = dept;
    }

    pub fn credit(&self) -> &[Vec<String>] {
        self.credit.as_ref()
    }

    pub fn set_credit(&mut self, credit: Vec<Vec<String>>) {
        self.credit = credit;
    }

    pub fn msn(&self) -> &str {
        self.msn.as_ref()
    }

    pub fn set_msn(&mut self, msn: String) {
        self.msn = msn;
    }
}
