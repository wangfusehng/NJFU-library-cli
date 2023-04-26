use super::ts::Ts;
use crate::utils::def;
use serde::{Deserialize, Serialize};
/// # Site struct
/// Site struct is used to store the information of the site.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    #[serde(rename = "devName")]
    dev_name: String,
    #[serde(rename = "devId")]
    dev_id: String,
    ts: Option<Vec<Ts>>,
}

impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dev_name: {}\ndev_id: {}\n", self.dev_name, self.dev_id)?;
        self.ts.as_ref().map(|ts| -> Result<(), std::fmt::Error> {
            for t in ts {
                write!(f, "{}", t)?;
                write!(f, "{}", def::LINE_SEPARATOR.as_str())?;
            }
            Ok(())
        });
        Ok(())
    }
}

impl Site {
    pub fn new(dev_name: String, dev_id: String, ts: Option<Vec<Ts>>) -> Self {
        Site {
            dev_name,
            dev_id,
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
}

/// tranform the site to the site id
pub fn get_site_id(site: String) -> Result<String, String> {
    let _floor = &site[0..4];
    match &site[4..].parse() {
        Ok(_site) => {
            let floor = def::ROOMS.get(_floor);
            match floor {
                Some(floor) => {
                    let id = floor.dev_start() + _site - 1;

                    if id >= floor.dev_start() && id <= floor.dev_end() {
                        Ok(id.to_string())
                    } else {
                        Err("parse room id error".into())
                    }
                }
                None => Err("parse room id error".into()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
