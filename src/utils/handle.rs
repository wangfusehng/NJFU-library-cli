use super::account;
use crate::error::RespError;
use crate::njfulib::resp::Data;
use crate::njfulib::resp::Resp;
use anyhow::anyhow;
use anyhow::Result;

pub async fn get_name_info(resp: Resp, query_name: String) -> Result<Resp> {
    if resp.code != 0 {
        return Err(anyhow!(RespError::Unknown(resp.message)));
    }
    let message = resp.message;
    let datas = resp.data.clone().unwrap();
    for data in datas {
        let devs = match data {
            Data::Site(site) => match site.resv_info {
                Some(devs) => devs.clone(),
                None => continue,
            },
            _ => panic!("no site info in response"),
        };
        for dev in devs {
            let resv_id = dev.resv_id;
            let data = account::get_account_by_resv_id(resv_id)
                .await?
                .data
                .clone()
                .unwrap();
            let sign_rec = match data[0].clone() {
                Data::SignRec(sign_rec) => sign_rec,
                _ => panic!("no sign record"),
            };
            if sign_rec.true_name == query_name {
                let data = Data::SignRec(sign_rec);
                return Ok(Resp::new(0, message.to_string(), Some(vec![data])));
            }
        }
    }
    Err(anyhow!(RespError::Nodata))
}

pub async fn get_site_info(resp: Resp, index: u32) -> Result<Resp> {
    if resp.code != 0 {
        return Err(anyhow!(RespError::Unknown(resp.message)));
    }
    let data = resp.data.clone().unwrap();
    let mut site = match data[index as usize - 1].clone() {
        Data::Site(site) => site,
        _ => panic!("no site info in response"),
    };
    let resv_id = site.resv_info.as_ref().unwrap()[0].resv_id.clone();
    let resv_name = account::get_name_by_resv_id(resv_id).await.unwrap();
    for item in site.resv_info.as_mut().unwrap().iter_mut() {
        item.resv_name = resv_name.clone();
    }

    Ok(Resp::new(
        0,
        resp.message.to_string(),
        Some(vec![Data::Site(site.clone())]),
    ))
}

pub fn handle_status(resp: Resp) -> Result<Resp> {
    if resp.code != 0 {
        return Err(anyhow!(RespError::Unknown(resp.message)));
    }
    let mut new_data = resp.data.clone().unwrap();
    new_data.reverse();
    Ok(Resp::new(resp.code, resp.message, Some(new_data)))
}
