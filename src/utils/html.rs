use crate::role::state::State;
use scraper::{ElementRef, Html, Selector};
use serde_json::Value;

pub fn parse_state(resp: Value) -> Result<Vec<State>, Box<dyn std::error::Error>> {
    let msg = resp["msg"].as_str().ok_or("no msg in response")?;
    let correct_html = msg.replace("tbody", "table");
    let msg = Html::parse_fragment(correct_html.as_str());

    let context_selector = Selector::parse("table")?;
    let context = msg.select(&context_selector);

    let mut ret: Vec<State> = Vec::new();

    for item in context {
        if let (Ok(name), Ok(start_time), Ok(end_time)) = (
            parse_name(&item),
            parse_start_time(&item),
            parse_end_time(&item),
        ) {
            let id = parse_id(&item)?;
            ret.push(State::new(id, name, start_time, end_time));
        }
    }
    Ok(ret)
}

fn parse_name(item: &ElementRef) -> Result<String, Box<dyn std::error::Error>> {
    let a_selector = Selector::parse(".box a")?;
    Ok(item
        .select(&a_selector)
        .nth(0)
        .ok_or("no name in item of response")?
        .inner_html())
}

fn parse_start_time(item: &ElementRef) -> Result<String, Box<dyn std::error::Error>> {
    let time_selector = Selector::parse(".text-primary")?;
    Ok(item
        .select(&time_selector)
        .nth(0)
        .ok_or("no start_time in item of response")?
        .inner_html())
}

fn parse_end_time(item: &ElementRef) -> Result<String, Box<dyn std::error::Error>> {
    let time_selector = Selector::parse(".text-primary")?;
    Ok(item
        .select(&time_selector)
        .nth(1)
        .ok_or("no end_time in item of response")?
        .inner_html())
}

fn parse_id(item: &ElementRef) -> Result<String, Box<dyn std::error::Error>> {
    let id_selector = Selector::parse(".text-center .click")?;
    let text_center_a = item.select(&id_selector).next();
    match text_center_a {
        Some(text_center_a) => {
            let rsv_id = text_center_a.value().attr("rsvid");
            match rsv_id {
                Some(id) => Ok(id.to_string()),
                None => {
                    let onclick = text_center_a.value().attr("onclick");
                    match onclick {
                        Some(onclick) => {
                            let id = onclick
                                .split("(")
                                .nth(1)
                                .ok_or("no id in onclick")?
                                .split(")")
                                .nth(0)
                                .ok_or("no id in onclick")?;
                            Ok(id.to_string())
                        }
                        None => Ok("none".to_string()),
                    }
                }
            }
        }
        None => Ok("none".to_string()),
    }
}
