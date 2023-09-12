use crate::def;
use crate::njfulib::resp::Data;
use crate::njfulib::resp::Resp;
use anyhow::Result;

pub fn get_account_by_resv_id(resv_id: u32) -> Result<Resp> {
    let url = format!("{}?resvId={}", def::SEARCHACCOUNT_URL, resv_id);
    Ok(def::CLIENT.get(url).send()?.json::<Resp>()?)
}

pub fn get_name_by_resv_id(resv_id: u32) -> Result<String> {
    let account = get_account_by_resv_id(resv_id)?;
    let data = account.data().clone().unwrap();
    let record = &data[0];
    let sign_rec = match record {
        Data::SignRec(sign_rec) => sign_rec,
        _ => panic!("no SignRec in record"),
    };
    Ok(sign_rec.true_name().to_string())
}
