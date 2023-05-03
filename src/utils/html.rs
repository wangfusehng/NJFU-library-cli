use std::io::{Read, Write};

use crate::role::state::State;
use anyhow::{anyhow, Context, Result};
use scraper::{ElementRef, Html, Selector};

pub fn parse_state(msg: String) -> Result<Vec<State>> {
    let correct_html = msg.replace("tbody", "table");
    let msg = Html::parse_fragment(correct_html.as_str());

    let context_selector = Selector::parse("table").expect("no table in response");
    let context = msg.select(&context_selector);

    let mut ret: Vec<State> = Vec::new();

    for item in context {
        if let (Ok(name), Ok(start_time), Ok(end_time)) = (
            parse_name(&item),
            parse_start_time(&item),
            parse_end_time(&item),
        ) {
            let id = parse_id(&item).context("no id in item of response")?;
            ret.push(State::new(id, name, start_time, end_time));
        }
    }
    Ok(ret)
}

pub fn parse_in(item: String) -> Result<String> {
    let html = Html::parse_document(item.as_str());
    let in_selector = Selector::parse("p").expect("no p in response");
    let vec = html.select(&in_selector).collect::<Vec<_>>();
    let state = vec[0].inner_html();
    if state == "操作成功" {
        // check in site
        Ok(format!("{}\n{}", state, vec[1].inner_html()))
    } else if state.contains("当前使用中") {
        // login in site but in use now
        let mut ret = String::new();
        ret.push_str(vec[0].inner_html().as_str());
        let str = vec[1].inner_html();
        let str = str.split(')').nth(1).context("no ) in response")?;
        ret.push_str(str);
        Err(anyhow!(ret))
    } else if state.contains("操作成功") {
        // login in site success
        let detail = vec[vec.len() - 1].inner_html();

        let html = Html::parse_fragment(detail.as_str());
        let opt_selector = Selector::parse("option").expect("no opt in response");
        let mut opt = html.select(&opt_selector);
        let opt = opt.next().context("no opt in response")?.inner_html();
        Ok(opt)
    } else if state.contains("操作失败") {
        // login in site but fail
        let mut ret = String::new();
        vec.iter()
            .for_each(|item| ret.push_str(item.inner_html().as_str()));
        Err(anyhow!(ret))
    } else {
        // other
        Err(anyhow!("parse state error"))
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
