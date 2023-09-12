use crate::def;
use serde::{Deserialize, Serialize};

/// # Floor information
#[derive(Debug, Serialize, Deserialize)]
pub struct Floor {
    room_id: u32,
    dev_start: u32,
    dev_end: u32,
    site_num: u32,
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, "room_id: {}", self.room_id)?;
        writeln!(f, "dev_start: {}", self.dev_start)?;
        writeln!(f, "dev_end: {}", self.dev_end)?;
        writeln!(f, "site_num: {}", self.site_num)?;
        Ok(())
    }
}

impl Floor {
    pub fn new(room_id: u32, dev_start: u32, dev_end: u32, site_num: u32) -> Self {
        Floor {
            room_id,
            dev_start,
            dev_end,
            site_num,
        }
    }

    pub fn room_id(&self) -> u32 {
        self.room_id
    }

    pub fn set_room_id(&mut self, room_id: u32) {
        self.room_id = room_id;
    }

    pub fn dev_start(&self) -> u32 {
        self.dev_start
    }

    pub fn set_dev_start(&mut self, dev_start: u32) {
        self.dev_start = dev_start;
    }

    pub fn dev_end(&self) -> u32 {
        self.dev_end
    }

    pub fn set_dev_end(&mut self, dev_end: u32) {
        self.dev_end = dev_end;
    }

    pub fn site_num(&self) -> u32 {
        self.site_num
    }

    pub fn set_site_num(&mut self, site_num: u32) {
        self.site_num = site_num;
    }
}
