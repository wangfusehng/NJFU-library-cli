use crate::cli::action::Action::{self, *};
use crate::cli::reserve::Reserve;
use crate::executor::*;
use crate::njfulib::resp::Resp;
use anyhow::{anyhow, Result};

pub fn handle_action(action: Action) -> Result<Resp> {
    // Perform the action.
    match action {
        Login {
            username,
            password,
            cookie,
        } => login(username, password, cookie),

        Status { day } => state(day),

        Query {
            day,
            name,
            site,
            filter,
        } => {
            if let Some(name) = name {
                query_by_name(day, name, filter)
            } else if let Some(site) = site {
                query_by_site(day, site)
            } else {
                Err(anyhow!("please input name or site"))
            }
        }

        Cancel { uuid } => cancel(uuid),

        Reserve(Reserve {
            sites,
            filter,
            day,
            start,
            end,
            retry,
        }) => reserve(sites, filter, day, start, end, retry),
    }
}
