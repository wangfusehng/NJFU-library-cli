use crate::def;
use crate::role::dev::Dev;
use crate::role::resp::Data;
use crate::role::resp::Resp;
use crate::role::resv::Resv;
use crate::role::site::Site;
use anyhow::{anyhow, Result};
use reqwest::blocking::Response;

pub fn get_account_by_resv_id(resv_id: u32) -> Result<Resp> {
    let url = format!("{}?resvId={}", def::SEARCHACCOUNT_URL, resv_id);
    Ok(def::CLIENT.get(url).send()?.json::<Resp>()?)
}

pub fn get_name_info(resp: Resp, query_name: String) -> Result<Resp> {
    let message = resp.message();
    let datas = resp.data().clone().unwrap();
    for data in datas {
        let devs = match data {
            Data::Site(site) => match site.resv_info() {
                Some(devs) => devs.clone(),
                None => continue,
            },
            _ => panic!("no site info in response"),
        };
        for dev in devs {
            let resv_id = dev.resv_id();

            let data = get_account_by_resv_id(resv_id)?.data().clone().unwrap();
            let sign_rec = match &data[0] {
                Data::SignRec(sign_rec) => sign_rec,
                _ => panic!("no user info in response"),
            };
            if sign_rec.true_name() == query_name {
                let resv_id = dev.resv_id();
                let dev_id = dev.dev_id();

                let site = Site::new(dev_id, resv_id, Some(vec![Dev::new(resv_id, dev_id)]));
                let data = Data::Site(site);
                return Ok(Resp::new(message.to_string(), Some(vec![data])));
            }
        }
    }
    Ok(Resp::new(String::from("no such user"), None))
}

pub fn get_site_info(resp: Resp, index: u32) -> Result<Resp> {
    let data = resp.data().clone().unwrap();
    let site = match &data[index as usize - 1] {
        Data::Site(site) => site,
        _ => panic!("no site info in response"),
    };

    Ok(Resp::new(
        resp.message().to_string(),
        Some(vec![Data::Site(site.clone())]),
    ))
}
