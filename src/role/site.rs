use super::ts::Ts;
use crate::utils::def;
use anyhow::{anyhow, Context, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    #[serde(rename = "devName")]
    dev_name: String,
    #[serde(rename = "devId")]
    dev_id: String,
    #[serde(rename = "labId")]
    lab_id: String,
    ts: Option<Vec<Ts>>,
}

impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "dev_name: {}", self.dev_name)?;
        writeln!(f, "-----")?;
        self.ts.as_ref().map(|ts| -> Result<(), std::fmt::Error> {
            for t in ts {
                writeln!(f, "{}", t)?;
                writeln!(f, "{}", def::LINE_SEPARATOR)?;
            }
            Ok(())
        });
        Ok(())
    }
}

impl Site {
    pub fn new(dev_name: String, dev_id: String, lab_id: String, ts: Option<Vec<Ts>>) -> Self {
        Site {
            dev_name,
            dev_id,
            lab_id,
            ts,
        }
    }

    pub fn set_dev_name(&mut self, dev_name: String) {
        self.dev_name = dev_name;
    }

    pub fn dev_name(&self) -> &str {
        self.dev_name.as_ref()
    }

    pub fn dev_id(&self) -> &str {
        self.dev_id.as_ref()
    }

    pub fn set_dev_id(&mut self, dev_id: String) {
        self.dev_id = dev_id;
    }

    pub fn ts(&self) -> Option<&Vec<Ts>> {
        self.ts.as_ref()
    }

    pub fn set_ts(&mut self, ts: Option<Vec<Ts>>) {
        self.ts = ts;
    }

    pub fn lab_id(&self) -> &str {
        self.lab_id.as_ref()
    }

    pub fn set_lab_id(&mut self, lab_id: String) {
        self.lab_id = lab_id;
    }
}

/// tranform the site or spacec to the site or space id
pub fn name_to_id(site: String) -> Result<u32> {
    split_site(site.clone()).or_else(|_| split_space(site))
}

/// tranform site or space id to site or space name
pub fn id_to_name(id: u32) -> Result<String> {
    // search space
    for (k, v) in def::SPACE.iter() {
        if id == *v {
            return Ok(k.to_string());
        }
    }
    // search site
    let mut floor = "";
    for (k, v) in def::ROOMS.iter() {
        if id >= v.dev_start() && id <= v.dev_end() {
            floor = k;
            break;
        }
    }
    if floor.is_empty() {
        anyhow::bail!("{} is not a site or space", id);
    }
    let site = id
        - def::ROOMS
            .get(floor)
            .context(format!("Failed to get floor {}", floor))?
            .dev_start()
        + 1;
    Ok(format!("{}{:03}", floor, site))
}

/// site id fiter by floor
pub fn site_fiter_by_floor(site: String, floor: Vec<String>) -> Result<bool> {
    let id = name_to_id(site)?;
    let mut floor_name = "";
    for i in floor.iter() {
        let v = def::ROOMS
            .get(i.as_str())
            .ok_or(anyhow!("parse room error"))?;

        if id >= v.dev_start() && id <= v.dev_end() {
            floor_name = i.as_str();
            break;
        }
    }
    if floor_name.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

/// get random site name in libray
pub fn get_random_site_name() -> Result<String> {
    let mut rng = rand::thread_rng();
    let floor_name = def::FLOOR
        .get(rng.gen_range(0..def::FLOOR.len()))
        .context("get floor name error")?;
    let floor = def::ROOMS.get(floor_name).context("get floor error")?;
    let site_id = rng.gen_range(floor.dev_start()..floor.dev_end() + 1);
    id_to_name(site_id)
}

pub fn split_space(dev_name: String) -> Result<u32> {
    if let "8A" = &dev_name[..2] {
        def::SPACE
            .get(&dev_name[..5])
            .cloned()
            .context(format!("Failed to get space for {}", dev_name))
    } else {
        anyhow::bail!("{} is not a space", dev_name)
    }
}

pub fn split_site(dev_name: String) -> Result<u32> {
    let floor = def::ROOMS
        .get(&dev_name[0..4])
        .context(format!("Failed to get floor for {}", &dev_name[0..4]))?;
    let site = &dev_name[4..].parse()?;
    let id = floor.dev_start() + site - 1;

    if id >= floor.dev_start() && id <= floor.dev_end() {
        Ok(id)
    } else {
        anyhow::bail!("{} is not a site", dev_name)
    }
}

#[test]
fn test_site() {
    let site = "5F-A100".to_string();
    let get_site = id_to_name(name_to_id(site.to_string()).unwrap()).unwrap();
    assert_eq!(site, get_site);
}

#[test]
fn test_space() {
    let space = "8A506".to_string();
    let ret = id_to_name(name_to_id(space.to_string()).unwrap()).unwrap();
    println!("{}", ret);
    assert_eq!(space, ret);
}
