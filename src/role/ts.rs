/// # Ts struct
/// Ts is a struct that contains the information of a time slot.
/// The information includes the owner of the time slot, the start time, the end time and the status of the time slot.
#[derive(Debug)]
pub struct Ts{
    owner: String,
    start: String,
    end: String,
    status: String,
}

impl Ts{
    pub fn new(owner: String, start: String, end: String, status: String) -> Self {
        Ts {
            owner,
            start,
            end,
            status,
        }
    }

    pub fn owner(&self) -> &str {
        self.owner.as_ref()
    }

    pub fn set_owner(&mut self, owner: String) {
        self.owner = owner;
    }

    pub fn start(&self) -> &str {
        self.start.as_ref()
    }

    pub fn set_start(&mut self, start: String) {
        self.start = start;
    }

    pub fn end(&self) -> &str {
        self.end.as_ref()
    }

    pub fn set_end(&mut self, end: String) {
        self.end = end;
    }

    pub fn status(&self) -> &str {
        self.status.as_ref()
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }
}
