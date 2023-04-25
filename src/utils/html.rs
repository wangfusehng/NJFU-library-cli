use crate::role::state::State;
use scraper::{Html, Selector};
use serde_json::Value;

pub fn parse_state(resp: Value) -> Option<Vec<State>> {
    let msg = Html::parse_fragment(resp["msg"].as_str()?);
    let context_selector = Selector::parse(".box").ok()?;
    let context = msg.select(&context_selector);

    let mut ret: Vec<State> = Vec::new();

    for item in context {
        let a_selector = Selector::parse(".box a").ok()?;
        let time_selector = Selector::parse(".text-primary").ok()?;
        let id_selector = Selector::parse(".click").ok()?;

        let site = item.select(&a_selector).nth(0)?.inner_html();
        let start_time = item.select(&time_selector).nth(0)?.inner_html();
        let end_time = item.select(&time_selector).nth(1)?.inner_html();
        let id_class = item.select(&id_selector).nth(0)?.html().to_string();

        match id_class.split("id=").nth(1) {
            Some(id) => {
                let id = id[1..10].to_string();
                ret.push(State::new(id, site, start_time, end_time));
            }
            None => {
                ret.push(State::new("doing".to_string(), site, start_time, end_time));
            }
        }
    }
    Some(ret)
}
