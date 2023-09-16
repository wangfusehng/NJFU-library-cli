use crate::def;
use crate::njfulib::resp::Data;
use crate::njfulib::resp::Resp;
use anyhow::Result;

pub async fn get_name_by_resv_id(resv_id: u32) -> Result<String> {
    let url = format!("{}?resvId={}", def::SEARCHACCOUNT_URL, resv_id);
    let resp = def::CLIENT.get(url).send().await?.json::<Resp>().await?;

    let data = resp.data.clone().unwrap();
    let record = &data[0];
    let sign_rec = match record {
        Data::SignRec(sign_rec) => sign_rec,
        _ => panic!("no SignRec in record"),
    };
    Ok(sign_rec.true_name.to_string())
}
