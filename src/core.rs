use crate::cli::Action::*;
use crate::cli::{Action, Reserve};
use crate::context;
use crate::utils::def;

/// Handle the cmd action.
pub fn handle_action(action: Action) {
    let context = context::Context::new();
    // Perform the action.
    match action {
        Query { day, name, site } => {
            println!("Result:");
            print!("{}", def::LINE_SEPARATOR.as_str());

            if name.is_some() {
                match context.query_by_name(day.clone(), name.unwrap()) {
                    Ok(result) => {
                        result.iter().for_each(|x| println!("{}", x));
                    }
                    Err(e) => println!("{}", e),
                }
            }

            if site.is_some() {
                match context.query_by_site(day, site.unwrap()) {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }

        Login { username, password } => {
            println!("Login result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.login(username, password) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e),
            }
        }

        State {} => {
            println!("State:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.status() {
                Ok(result) => {
                    println!("id\t\tsite\t\tstart_time\t\tend_time");
                    result.iter().for_each(|x| println!("{}", x));
                }
                Err(e) => println!("{}", e),
            }
        }

        Cancel { id } => {
            println!("Cancel result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.cancel(id) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e),
            }
        }

        Reserve(Reserve {
            site,
            day,
            start,
            end,
        }) => {
            println!("Reserve result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.reserve(site, day, start, end) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e),
            }
        }

        In { site } => {
            println!("check in is not supported yet.")
        }

        Out { id } => {
            println!("Check out result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.check_out(id) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e),
            }
        }
    };
}
