use super::def;

#[derive(Debug)]
pub struct Floor(pub u32, pub u32, pub u32, pub u32);

impl Floor {
    pub fn new(room_id: u32, dev_start: u32, dev_end: u32, site_num: u32) -> Self {
        Floor(room_id, dev_start, dev_end, site_num)
    }
}

pub fn get_site_id(site: String) -> Result<String, Box<dyn std::error::Error>> {
    let _floor = &site[0..4];
    match &site[4..].parse() {
        Ok(_site) => {
            let floor = def::ROOMS.get(_floor);
            match floor {
                Some(floor) => {
                    let id = floor.1 + _site - 1;
                    if id >= floor.2 && id <= floor.3 {
                        Ok(id.to_string())
                    } else {
                        Err("parse room id error".into())
                    }
                }
                None => Err("parse room id error".into()),
            }
        }
        Err(_) => Err("parse site id error".into()),
    }
}
