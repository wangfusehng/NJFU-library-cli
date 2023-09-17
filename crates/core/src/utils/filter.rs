use crate::def;
use crate::njfulib::floor;
use crate::njfulib::floor::Floor;
use anyhow::Result;

pub fn handle_filter(filter: Option<Vec<String>>) -> Result<Vec<Floor>> {
    match filter {
        Some(filter) => {
            let mut ret = Vec::new();
            for item in filter.iter() {
                ret.push(floor::get_floor_by_room_name(item)?);
            }
            Ok(ret)
        }
        None => Ok(def::FLOORS.to_vec()),
    }
}
