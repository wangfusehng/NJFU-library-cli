use crate::cli::{Action, Reserve};
use crate::cli::Action::*;
use crate::context;
use log::*;

/// # Handle the cmd action.
pub fn handle_action(action: Action) {
    debug!("Action: {:?}", action);

    let context = context::Context::new();
    // Perform the action.
    match action {
        Query { name, site } => {
            if name.is_some() {
                match context.query_by_name(name.unwrap()) {
                    Some(result) => println!("{:#?}", result),
                    None => println!("No such student in library."),
                }
            } else if site.is_some() {
                match context.query_by_site(site.unwrap()) {
                    Some(result) => println!("{:#?}", result),
                    None => println!("No student owner the site."),
                }
            }
        }

        Login { username, password } => match context.login(username, password) {
            Some(result) => println!("{:#?}", result),
            None => println!("Login failed."),
        },

        State {} => match context.status() {
            Some(result) => println!("{:#?}", result),
            None => println!("no data."),
        },

        Cancel { id } => match context.cancel(id) {
            Some(result) => println!("{:#?}", result),
            None => println!("Cancel failed."),
        },

        Reserve(Reserve {
            site,
            day,
            start,
            end,
        }) => match context.reserve(site, day, start, end) {
            Some(result) => println!("{:#?}", result),
            None => println!("Reserve failed."),
        },
    };
}
