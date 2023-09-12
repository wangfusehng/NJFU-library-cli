use super::def;
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

        Query { day, name, site } => {
            if let Some(name) = name {
                query_by_name(day, name)
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
        }) => {
            // default is def::FLOOR
            let floor = match filter {
                Some(f) => f,
                None => def::FLOOR
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>(),
            };

            reserve(sites, floor, day, start, end, retry)
        }
    }
}
