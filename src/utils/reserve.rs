use super::site;
use super::student;

#[derive(Debug)]
pub struct Reserve {
    student: student::Student,
    site: site::Site,
}

impl Reserve {
    fn new(student: student::Student, site: site::Site) -> Self {
        Reserve { student, site }
    }
}
