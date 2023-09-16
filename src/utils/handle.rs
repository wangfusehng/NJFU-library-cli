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
    let datas = resp.data.unwrap();
    for mut data in datas {
        let resv_infos = match &mut data {
            Data::Site(site) => match &mut site.resv_info {
                Some(resv_infos) => resv_infos,
                None => continue,
            },
            _ => panic!("no site info in response"),
        };
        for resv_info in resv_infos {
            let resv_id = resv_info.resv_id;
            let true_name = account::get_name_by_resv_id(resv_id).await?;
            if true_name == query_name {
                resv_info.true_name = Some(true_name);
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
    let data = &mut resp.data.unwrap()[index as usize - 1];
    let site = match data {
        Data::Site(site) => site,
        _ => panic!("no site info in response"),
    };
    match &mut site.resv_info {
        Some(resv_infos) => {
            for resv_info in resv_infos.iter_mut() {
                let resv_id = resv_info.resv_id;
                let true_name = account::get_name_by_resv_id(resv_id).await?;
                resv_info.true_name = Some(true_name);
            }
        }
        _ => panic!("no site info in response"),
    };

    Ok(Resp::new(
        0,
        resp.message.to_string(),
        Some(vec![data.clone()]),
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
