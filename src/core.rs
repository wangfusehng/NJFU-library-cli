use crate::cli::action::Action::{self, *};
use crate::cli::infomation::Infomation;
use crate::cli::reserve::Reserve;
use crate::executor::*;
use crate::role::login;
use crate::utils::def;
use ferris_says::say;
use std::io::{stdout, BufWriter};

/// Handle the cmd action.
pub fn handle_action(action: Action) {
    // Perform the action.
    match action {
        Query { day, name, site } => {
            println!("Result:");
            println!("{}", def::LINE_SEPARATOR);

            if name.is_some() {
                match query_by_name(day.clone(), name.unwrap()) {
                    Ok(result) => {
                        result.iter().for_each(|x| println!("{}", x));
                    }
                    Err(e) => panic!("{}", e),
                }
            }

            if site.is_some() {
                match query_by_site(day, site.unwrap()) {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        }

        Login { username, password } => {
            println!("Login result:");
            println!("{}", def::LINE_SEPARATOR);
            match login(username, password) {
                Ok(result) => println!("{}", result),
                Err(e) => panic!("{}", e),
            }
        }

        State {} => {
            println!("State:");
            println!("{}", def::LINE_SEPARATOR);
            match state() {
                Ok(result) => {
                    println!("site\t\tstart_time\t\tend_time\t\tid");
                    result.iter().for_each(|x| println!("{}", x));
                }
                Err(e) => panic!("{}", e),
            }
        }

        Cancel { id } => {
            println!("Cancel result:");
            println!("{}", def::LINE_SEPARATOR);
            match cancel(id) {
                Ok(result) => println!("{}", result),
                Err(e) => panic!("{}", e),
            }
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

            match reserve(sites, floor, day, start, end, retry) {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }
        }

        In { site, time } => {
            println!("Check in result:");
            println!("{}", def::LINE_SEPARATOR);
            match check_in(site, time) {
                Ok(result) => println!("{}", result),
                Err(e) => panic!("{}", e),
            }
        }

        Out { id } => {
            println!("Check out result:");
            println!("{}", def::LINE_SEPARATOR);
            match check_out(id) {
                Ok(result) => println!("{}", result),
                Err(e) => panic!("{}", e),
            }
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
                    say(&author, 24, writer).expect("say failed.")
                }
                Infomation::User => {
                    let mut login = login::Login::new("".to_string(), "".to_string());
                    login.read_from_file().expect("read config error");
                    println!("{}", login);
                }
            }
        }
    };
}
