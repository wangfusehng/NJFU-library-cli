#[derive(Debug)]
pub struct Site {
    dev_name: String,
    dev_id: String,
    // # Owner
    // Owner is a tuple struct
    // ---
    // owner String
    // start String
    // end String
    // status String
    ts: Option<Vec<(String, String, String, String)>>,
}

impl Site {
    pub fn new(
        dev_name: String,
        dev_id: String,
        ts: Option<Vec<(String, String, String, String)>>,
    ) -> Self {
        Site {
            dev_name,
            dev_id,
            ts,
        }
    }
}
