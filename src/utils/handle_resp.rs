use super::account;
use crate::role::resp::Data;
use crate::role::resp::Resp;
use anyhow::Result;

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
            let data = account::get_account_by_resv_id(resv_id)?
                .data()
                .clone()
                .unwrap();
            let sign_rec = match data[0].clone() {
                Data::SignRec(sign_rec) => sign_rec,
                _ => panic!("no sign record"),
            };
            if sign_rec.true_name() == query_name {
                let data = Data::SignRec(sign_rec);
                return Ok(Resp::new(0, message.to_string(), Some(vec![data])));
            }
        }
    }
    Ok(Resp::new(1, String::from("no such user"), None))
}

pub fn get_site_info(resp: Resp, index: u32) -> Result<Resp> {
    let data = resp.data().clone().unwrap();
    let site = match &data[index as usize - 1] {
        Data::Site(site) => site,
        _ => panic!("no site info in response"),
    };

    Ok(Resp::new(
        0,
        resp.message().to_string(),
        Some(vec![Data::Site(site.clone())]),
    ))
}
