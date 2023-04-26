use crate::cli::Action::*;
use crate::cli::{Action, Reserve};
use crate::context;
use crate::utils::def;
use log::*;

/// # Handle the cmd action.
pub fn handle_action(action: Action) {
    debug!("Action: {:?}", action);

    let context = context::Context::new();
    // Perform the action.
    match action {
        Query { day, name, site } => {
            println!("Result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            if name.is_some() {
                match context.query_by_name(day, name.unwrap()) {
                    Some(result) => {
                        result.iter().for_each(|x| println!("{}", x));
                    }
                    None => println!("No such student in library."),
                }
            } else if site.is_some() {
                match context.query_by_site(day, site.unwrap()) {
                    Some(result) => {
                        println!("{}", result);
                    }
                    None => println!("no site info"),
                }
            }
        }

        Login { username, password } => {
            println!("Login result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.login(username, password) {
                Some(result) => println!("{}", result),
                None => println!("Login failed."),
            }
        }

        State {} => {
            println!("State:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.status() {
                Some(result) => {
                    println!("id\t\tsite\t\tstart_time\t\tend_time");
                    result.iter().for_each(|x| println!("{}", x));
                }
                None => println!("no state data."),
            }
        }

        Cancel { id } => {
            println!("Cancel result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.cancel(id) {
                Some(result) => println!("{}", result),
                None => println!("Cancel failed."),
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
                Some(result) => println!("{}", result),
                None => println!("Reserve failed."),
            }
        }

        In { site } => {
            println!("check in is not supported yet.")
        }

        Out { id } => {
            println!("Check out result:");
            print!("{}", def::LINE_SEPARATOR.as_str());
            match context.check_out(id) {
                Some(result) => println!("{}", result),
                None => println!("Check out failed."),
            }
        }
    };
}
