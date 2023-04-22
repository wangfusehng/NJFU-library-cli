#[derive(Debug)]
pub struct Floor(pub u32, pub u32, pub u32, pub u32);

impl Floor {
    pub fn new(room_id: u32, dev_start: u32, dev_end: u32, site_num: u32) -> Self {
        Floor(room_id, dev_start, dev_end, site_num)
    }
}
