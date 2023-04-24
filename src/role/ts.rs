/// # Ts struct
/// Ts is a struct that contains the information of a time slot.
/// The information includes the owner of the time slot, the start time, the end time and the status of the time slot.
#[derive(Debug)]
pub struct Ts{
    pub owner: String,
    pub start: String,
    pub end: String,
    pub status: String,
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
}
