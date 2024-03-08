use chrono::{Local, Datelike};

struct Today {
    year: i32,
    month: u32,
    day: u32,
    wod: &'static str,
}

fn get_wod(n: u32) -> &'static str {
    match n {
        1 => "月",
        2 => "火",
        3 => "水",
        4 => "木",
        5 => "金",
        6 => "土",
        _ => "日",
    }
}

pub fn run() -> Result<(), String> {
    let now = Local::now();

    let today: Today = Today {
        year: now.year(),
        month: now.month(),
        day: now.day(),
        wod: get_wod(now.weekday().number_from_monday()),
    };

    match get_reiwa(today.year, today.month) {
        Err(err) => return Err(err),
        Ok(reiwa) => println!("令和{:2}年{:2}月{:2}日 {}曜日", reiwa, today.month, today.day, today.wod),
    }
    Ok(())
}

fn get_reiwa(year: i32, month: u32) -> Result<i32, String> {
    const BASE: i32 = 2018;
    if (year >= 2019 && month >= 5) || (year >= 2020) { return Ok(year - BASE) }
    Err("Error: Before Reiwa".to_string())
}
