use crate::def;
use crate::error::ReserveError;
use crate::error::RespError;
use crate::njfulib::resp::Data;
use crate::njfulib::resp::Resp;
use anyhow::anyhow;
use anyhow::Result;
use futures::future::join_all;

pub fn get_resv_task(
    resv_id: u32,
) -> impl std::future::Future<Output = std::result::Result<reqwest::Response, reqwest::Error>> {
    let url = format!("{}?resvId={}", def::SEARCHACCOUNT_URL, resv_id);
    def::CLIENT.get(url).send()
}

pub async fn get_name_info(resp: Resp, query_name: String) -> Result<Resp> {
    if resp.code != 0 {
        return Err(anyhow!(RespError::Unknown(resp.message)));
    }

    // generate query tasks
    let mut resv_tasks = vec![];
    resp.data.unwrap().into_iter().for_each(|data| {
        match data {
            Data::Site(site) => match site.resv_info {
                Some(resv_infos) => {
                    resv_tasks.extend(
                        resv_infos
                            .into_iter()
                            .map(|resv_info| get_resv_task(resv_info.resv_id)),
                    );
                }
                None => panic!("no site info in response"),
            },
            _ => panic!("no site info in response"),
        };
    });

    // query by resv_id
    let resp_tasks = join_all(resv_tasks)
        .await
        .into_iter()
        .try_fold(vec![], |mut acc, resp| match resp {
            Ok(resp) => {
                acc.push(resp.json::<Resp>());
                Ok(acc)
            }
            Err(e) => return Err(anyhow!(e)),
        })?;

    // handle resp
    let ret_data = join_all(resp_tasks)
        .await
        .into_iter()
        .try_fold(vec![], |mut acc, item| match item {
            Ok(resp) => {
                if resp.code == 0 {
                    match &resp.data.clone().unwrap()[0] {
                        Data::SignRec(sign_rec) => {
                            if sign_rec.true_name == query_name {
                                acc.push(Data::SignRec(sign_rec.clone()));
                            }
                        }
                        _ => panic!("no account info in response"),
                    };
                }
                Ok(acc)
            }
            Err(e) => return Err(anyhow!(e)),
        })?;

    if ret_data.len() > 0 {
        return Ok(Resp::new(0, resp.message, Some(ret_data)));
    } else {
        Err(anyhow!(RespError::Nodata))
    }
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
                return Ok(get_resv_task(resv_id).await?.json::<Resp>().await?);
            }
        }
        _ => panic!("no site info in response"),
    };

    Err(anyhow!(RespError::Nodata))
}

pub fn handle_status(resp: Resp) -> Result<Resp> {
    if resp.code != 0 {
        return Err(anyhow!(RespError::Unknown(resp.message)));
    }
    let mut new_data = resp.data.clone().unwrap();
    new_data.reverse();
    let new_message = resp.message
        + "\n  dev    status           begin                 end                       uuid";
    Ok(Resp::new(resp.code, new_message, Some(new_data)))
}

pub fn handle_reserve(resp: Resp) -> Result<Resp> {
    if resp.code != 0 {
        if resp.message.contains("设备在该时间段内已被预约") {
            return Err(anyhow!(RespError::Reserve(
                ReserveError::SiteAlreadReserved
            )));
        } else {
            return Err(anyhow!(RespError::Reserve(ReserveError::Unknown(
                resp.message
            ))));
        }
    }
    Ok(resp)
}
