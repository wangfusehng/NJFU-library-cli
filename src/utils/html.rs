use std::io::{Read, Write};

use crate::role::state::State;
use anyhow::{anyhow, Context, Result};
use scraper::{ElementRef, Html, Selector};

// parse state html in response
pub fn parse_state(msg: String) -> Result<Vec<State>> {
    let correct_html = msg.replace("tbody", "table");
    let msg = Html::parse_fragment(correct_html.as_str());

    let context_selector = Selector::parse("table").expect("no table in response");
    let context = msg.select(&context_selector);

    let mut ret: Vec<State> = Vec::new();

    for item in context {
        if let (Ok(name), Ok(start_time), Ok(end_time), Ok(state)) = (
            parse_name(&item),
            parse_start_time(&item),
            parse_end_time(&item),
            parse_site_state(&item),
        ) {
            let id = parse_id(&item).context("no id in item of response")?;
            ret.push(State::new(id, name, start_time, end_time, state));
        }
    }
    Ok(ret)
}

#[test]
fn test1() {
    use std::fs::File;
    let mut file = File::open("resp/center_undo.json").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let str: serde_json::Value = serde_json::from_str(&str).unwrap();
    println!(
        "{:#?}",
        parse_state(str["msg"].as_str().unwrap().to_string())
    );
}

// parse html in site login response
pub fn parse_site_login(item: String) -> Result<u32> {
    let html = Html::parse_document(item.as_str());
    let in_selector = Selector::parse("p").expect("no p in response");
    let vec = html.select(&in_selector).collect::<Vec<_>>();
    let state = vec[0].inner_html();

    if state == "【 登录成功 】" {
        // check in when a reserve is at time
        Ok(0)
    } else if state == "【 操作成功 】" {
        // check in without a reserve before
        let detail = vec[vec.len() - 1].inner_html();
        let html = Html::parse_fragment(detail.as_str());
        let option_selector = Selector::parse("option").expect("no opt in response");
        let mut option = html.select(&option_selector);

        let time = option
            .next()
            .context("no option in p")?
            .value()
            .attr("value")
            .context("no value in option")?
            .parse::<u32>()
            .context("parse u32 error")?;

        Ok(time)
    } else if state == "【 当前使用中 】" {
        // check in when the site is in use by others
        let mut ret = String::new();
        ret.push_str(state.as_str());
        ret.push(' ');
        let str = vec[1].inner_html();
        let str = str.split(')').nth(1).context("no ) in response")?;
        ret.push_str(str);
        Err(anyhow!(ret))
    } else if state == "操作失败" {
        let mut ret = String::new();
        vec.into_iter().for_each(|x| {
            ret.push_str(x.inner_html().as_str());
            ret.push(' ');
        });
        Err(anyhow!(ret))
    } else {
        let mut ret = String::new();
        vec.into_iter().for_each(|x| {
            ret.push_str(x.inner_html().as_str());
            ret.push(' ');
        });
        Err(anyhow!(ret))
    }
}

// parse check in site response
pub fn parse_in(item: String) -> Result<String> {
    let html = Html::parse_document(item.as_str());
    let in_selector = Selector::parse("p").expect("no p in response");
    let vec = html.select(&in_selector).collect::<Vec<_>>();
    let state = vec[0].inner_html();

    let mut ret = String::new();
    vec.iter().for_each(|item| {
        ret.push_str(item.inner_html().as_str());
        ret.push(' ');
    });
    if state == "操作成功" {
        Ok(ret)
    } else if state == "操作失败" {
        Err(anyhow!(ret))
    } else {
        Err(anyhow!(ret))
    }
}

fn parse_name(item: &ElementRef) -> Result<String> {
    let a_selector = Selector::parse(".box a").expect("no .box a in response");
    Ok(item
        .select(&a_selector)
        .next()
        .context("no name in item of response")?
        .inner_html())
}

fn parse_start_time(item: &ElementRef) -> Result<String> {
    let time_selector = Selector::parse(".text-primary").expect("no .text-primary in response");
    Ok(item
        .select(&time_selector)
        .next()
        .context("no start_time in item of response")?
        .inner_html())
}

fn parse_end_time(item: &ElementRef) -> Result<String> {
    let time_selector = Selector::parse(".text-primary").expect("no .text-primary in response");
    Ok(item
        .select(&time_selector)
        .nth(1)
        .context("no end_time in item of response")?
        .inner_html())
}

fn parse_id(item: &ElementRef) -> Result<String> {
    let id_selector =
        Selector::parse(".text-center .click").expect("no .text-center .click in response");
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
                                .split('(')
                                .nth(1)
                                .context("no id in onclick")?
                                .split(')')
                                .next()
                                .context("no id in onclick")?;
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

fn parse_site_state(item: &ElementRef) -> Result<String> {
    let span_selector = Selector::parse("div span").expect("no span in response");
    let span = item.select(&span_selector).collect::<Vec<_>>();
    let mut ret = String::new();
    // judge size with 3
    let start = if span.len() >= 3 { span.len() - 3 } else { 0 };
    span[start..].iter().for_each(|i| {
        ret.push_str(&i.inner_html());
        ret.push(' ');
    });
    Ok(ret)
}
