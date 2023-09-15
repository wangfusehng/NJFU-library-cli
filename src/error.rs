use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("config error")]
    Config,
    #[error("input error: {0}")]
    InputError(String),
    #[error("site error")]
    SiteError,
    #[error("floor error")]
    RoomError,
}

#[derive(Error, Debug)]
pub enum RespError {
    #[error("no data in response")]
    Nodata,
    #[error("unknown error: {0}")]
    Unknown(String),
}
