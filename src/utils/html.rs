use crate::role::state::State;
use scraper::{Html, Selector};
use serde_json::Value;

pub fn parse_state(resp: Value) -> Result<Vec<State>, Box<dyn std::error::Error>> {
    let msg = Html::parse_fragment(resp["msg"].as_str().unwrap_or(""));
    let context_selector = Selector::parse(".box")?;
    let context = msg.select(&context_selector);

    let mut ret: Vec<State> = Vec::new();

    for item in context {
        let a_selector = Selector::parse(".box a")?;
        let time_selector = Selector::parse(".text-primary").unwrap();
        let id_selector = Selector::parse(".click").unwrap();

        let site = item
            .select(&a_selector)
            .nth(0)
            .expect("no site")
            .inner_html();
        let start_time = item
            .select(&time_selector)
            .nth(0)
            .expect("no start_time")
            .inner_html();
        let end_time = item
            .select(&time_selector)
            .nth(1)
            .expect("no end_time")
            .inner_html();
        let id_class = item
            .select(&id_selector)
            .nth(0)
            .expect("no id")
            .value()
            .attr("rsvid");

        match id_class {
            Some(id) => {
                ret.push(State::new(id.to_string(), site, start_time, end_time));
            }
            None => {
                let onclick_class = item
                    .select(&id_selector)
                    .nth(0)
                    .expect("no id")
                    .value()
                    .attr("onclick");
                match onclick_class {
                    Some(onclick) => {
                        let id = onclick
                            .split("finish(")
                            .nth(1)
                            .unwrap()
                            .split(")")
                            .nth(0)
                            .unwrap();
                        ret.push(State::new(id.to_string(), site, start_time, end_time));
                    }
                    None => {
                        ret.push(State::new("none".to_string(), site, start_time, end_time));
                    }
                }
            }
        }
    }
    Ok(ret)
}

#[test]
fn htmltest() {
    use std::fs::File;
    use std::io::Read;
    let file =
        File::open("C:/Users/jyf/code/NJFU-library-cli/resp/center_with_now_reserve.json").unwrap();
    let str: String = file.bytes().map(|x| x.unwrap() as char).collect();
    let resp: Value = serde_json::from_str(&str).unwrap();
    let ret = parse_state(resp.clone());
    println!("{:#?}", ret);
}
