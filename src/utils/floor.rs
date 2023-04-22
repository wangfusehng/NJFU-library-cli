use super::def;

#[derive(Debug)]
pub struct Floor(pub u32, pub u32, pub u32, pub u32);

impl Floor {
    pub fn new(room_id: u32, dev_start: u32, dev_end: u32, site_num: u32) -> Self {
        Floor(room_id, dev_start, dev_end, site_num)
    }
}


pub fn get_site_id(site: String) -> u32 {
    let _floor = &site[0..4];
    match &site[4..].parse() {
        Ok(_site) => {
            let floor = def::ROOMS.get(_floor);
            match floor {
                Some(floor) => floor.1 + _site - 1,
                None => {
                    panic!("no such site");
                }
            }
        }
        Err(e) => {
            panic!("parse room id error: {}", e);
        }
    }
}
