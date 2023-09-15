use crate::cli::action::Action::{self, *};
use crate::cli::reserve::Reserve;
use crate::error::ClientError;
use crate::executor::*;
use crate::njfulib::resp::Resp;
use anyhow::{anyhow, Result};

pub async fn handle_action(action: Action) -> Result<Resp> {
    // Perform the action.
    match action {
        Login {
            username,
            password,
            cookie,
        } => login(username, password, cookie).await,

        Status { day } => state(day).await,

        Query {
            day,
            name,
            site,
            filter,
        } => {
            if let Some(name) = name {
                query_by_name(day, name, filter).await
            } else if let Some(site) = site {
                query_by_site(day, site).await
            } else {
                Err(anyhow!(ClientError::InputError(
                    "no query name or site".to_string()
                )))
            }
        }

        Cancel { uuid } => cancel(uuid).await,

        Reserve(Reserve {
            sites,
            filter,
            day,
            start,
            end,
            retry,
        }) => reserve(sites, filter, day, start, end, retry).await,
    }
}
