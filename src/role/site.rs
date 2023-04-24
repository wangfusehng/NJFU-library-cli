use super::ts::Ts;
/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug)]
pub struct Site {
    dev_name: String,
    dev_id: String,
    ts: Option<Vec<Ts>>,
}

impl Site {
    pub fn new(dev_name: String, dev_id: String, ts: Option<Vec<Ts>>) -> Self {
        Site {
            dev_name,
            dev_id,
            ts,
        }
    }
}
