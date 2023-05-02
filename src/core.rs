use crate::cli::action::Action::{self, *};
use crate::cli::infomation::Infomation;
use crate::cli::reserve::Reserve;
use crate::executor::*;
use crate::role::login;
use crate::utils::def;
use anyhow::{Context, Ok, Result};
use ferris_says::say;
use std::io::{stdout, BufWriter};

/// Handle the cmd action.
pub fn handle_action(action: Action) -> Result<()> {
    // Perform the action.
    match action {
        Query { day, name, site } => {
            println!("Result:");
            println!("{}", def::LINE_SEPARATOR);

            if name.is_some() {
                query_by_name(day.clone(), name.unwrap())
                    .map(|result| {
                        result.iter().for_each(|x| println!("{}", x));
                    })
                    .context("query_by_name failed")?;
            }

            if site.is_some() {
                query_by_site(day, site.unwrap())
                    .map(|result| {
                        println!("{}", result);
                    })
                    .context("query_by_site failed")?;
            }
        }

        Login { username, password } => {
            println!("Login result:");
            println!("{}", def::LINE_SEPARATOR);
            login(username, password)
                .map(|result| {
                    println!("{}", result);
                })
                .context("login failed")?;
        }

        State {} => {
            println!("State:");
            println!("{}", def::LINE_SEPARATOR);
            state()
                .map(|result| {
                    println!("site\t\tstart_time\t\tend_time\t\tid");
                    result.iter().for_each(|x| println!("{}", x));
                })
                .context("state failed")?;
        }

        Cancel { id } => {
            println!("Cancel result:");
            println!("{}", def::LINE_SEPARATOR);
            cancel(id)
                .map(|result| println!("{}", result))
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
            println!("Reserve result:");
            println!("{}", def::LINE_SEPARATOR);

            // default is def::FLOOR
            let floor = match filter {
                Some(f) => f,
                None => def::FLOOR
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>(),
            };

            reserve(sites, floor, day, start, end, retry).map(|result| {
                println!("{}", result);
            })?;
        }

        In { site, time } => {
            println!("Check in result:");
            println!("{}", def::LINE_SEPARATOR);
            check_in(site, time).map(|result| println!("{}", result))?;
        }

        Out { id } => {
            println!("Check out result:");
            println!("{}", def::LINE_SEPARATOR);
            check_out(id).map(|result| println!("{}", result))?;
        }

        Info { infomation } => {
            println!("Info result:");
            println!("{}", def::LINE_SEPARATOR);
            match infomation {
                Infomation::Floor => {
                    println!("name\tstart\t\tend\t\tnumber");
                    for (name, floor) in def::ROOMS.iter() {
                        println!("{}\t{}", name, floor);
                    }
                }
                Infomation::Author => {
                    let author = String::from("author: 蒋雨峰\n南京林业大学20级本科生\n");
                    let writer = BufWriter::new(stdout());
                    say(&author, 24, writer).context("say failed.")?
                }
                Infomation::User => {
                    let mut login = login::Login::new("".to_string(), "".to_string());
                    login.read_from_file().context("read config error")?;
                    println!("{}", login);
                }
            }
        }
    };
    Ok(())
}
