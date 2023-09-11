use super::def;
use crate::cli::action::Action::{self, *};
use crate::cli::reserve::Reserve;
use crate::executor::*;
use anyhow::{anyhow, Context, Result};
use std::io::{stdout, BufWriter};

pub fn handle_action(action: Action) -> Result<()> {
    // Perform the action.
    match action {
        Login {
            username,
            password,
            cookie,
        } => {
            login(username, password, cookie)
                .map(|result| {
                    println!("{:#?}", result);
                })
                .context("login failed")?;
        }

        Status {} => {
            state()
                .map(|result| {
                    println!("{:#?}", result);
                })
                .context("state failed")?;
        }

        Query { day, name, site } => {
            if name.is_some() {
                query_by_name(day, name.unwrap())
                    .map(|result| {
                        println!("{:#?}", result);
                    })
                    .context("query_by_site failed")?;
            }

            if site.is_some() {
                query_by_site(day, site.unwrap())
                    .map(|result| {
                        println!("{:#?}", result);
                    })
                    .context("query_by_site failed")?;
            }
        }

        Cancel { uuid } => {
            cancel(uuid)
                .map(|result| println!("{:#?}", result))
                .context("cancel failed")?;
        }

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

            reserve(sites, floor, day, start, end, retry).map(|result| {
                println!("{:#?}", result);
            })?;
        }
    };
    Ok(())
}
