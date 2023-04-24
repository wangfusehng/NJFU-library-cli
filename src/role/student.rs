/// # Student struct
/// Student struct is used to store student's information
/// The information includes the name and the id of the student
#[derive(Debug)]
pub struct Student {
    name: String,
    id: String,
}

impl Student {
    pub fn new(name: String, id: String) -> Self {
        Student { name, id }
    }
}
