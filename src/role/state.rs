/// # State struct
/// State struct is used to store the information of the user's state.
/// 
/// # parameters
///    id: the id of the use
///    site: the site of the use
///    start_time: the start time of the user
///    end_time: the end time of the user
#[derive(Debug)]
pub struct State {
    pub id: String,
    pub site: String,
    pub start_time: String,
    pub end_time: String,
}

impl State {
    pub fn new(id: String, site: String, start_time: String, end_time: String) -> Self {
        State {
            id,
            site,
            start_time,
            end_time,
        }
    }
}
