use crate::cli::Action::*;
use crate::cli::{Action, Reserve};
use crate::context;
use log::*;

/// # Handle the cmd action.
pub fn handle_action(action: Action) {
    debug!("Action: {:?}", action);

    let context = context::Context::new();
    // Perform the action.
    match action {
        Query { day, name, site } => {
            if name.is_some() {
                match context.query_by_name(day, name.unwrap()) {
                    Some(result) => {
                        result.iter().for_each(|x| println!("{}", x));
                    }
                    None => println!("No such student in library."),
                }
            } else if site.is_some() {
                match context.query_by_site(day, site.unwrap()) {
                    Some(result) => println!("{}", result),
                    None => println!("no site info"),
                }
            }
        }

        Login { username, password } => match context.login(username, password) {
            Some(result) => println!("{}", result),
            None => println!("Login failed."),
        },

        State {} => match context.status() {
            Some(result) => {
                result.iter().for_each(|x| println!("{}", x));
            }
            None => println!("no state data."),
        },

        Cancel { id } => match context.cancel(id) {
            Some(result) => println!("{}", result),
            None => println!("Cancel failed."),
        },

        Reserve(Reserve {
            site,
            day,
            start,
            end,
        }) => match context.reserve(site, day, start, end) {
            Some(result) => println!("{}", result),
            None => println!("Reserve failed."),
        },

        In { site } => {
            println!("check in is not supported yet.")
        }

        Out { id } => match context.check_out(id) {
            Some(result) => println!("{}", result),
            None => println!("Check out failed."),
        },
    };
}
