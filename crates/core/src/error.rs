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
pub enum ReserveError {
    #[error("the site has been reserved")]
    SiteAlreadReserved,
    #[error("{0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub enum RespError {
    #[error("[no data from response]")]
    Nodata,
    #[error("[reserve error] {0}")]
    Reserve(ReserveError),
    #[error("[unknown error] {0}")]
    Unknown(String),
    #[error("[http error] {0}")]
    Reqwest(#[from] reqwest::Error),
}
