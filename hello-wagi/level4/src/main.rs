// The Wordle "solver" algorithm is based on a TypeScript sample written
// by @blaumeise20 (https://github.com/blaumeise20/blaumeise20). Thank
// you for allowing me to use it for this demo.

use std::str::from_utf8;
use anyhow::{Result, anyhow};
use http::{request, StatusCode};
use regex::{Captures, Regex};

use chrono::prelude::*;

const URL: &str = "https://www.nytimes.com/games/wordle/index.html";

fn get<S: AsRef<str>>(url: S) -> Result<String> {
    let req = request::Builder::new().uri(url.as_ref()).body(None)?;
    let mut res = wasi_experimental_http::request(req)?;
    match res.status_code {
        StatusCode::OK => Ok(from_utf8(&res.body_read_all()?)?.to_string()),
        _ => Err(anyhow!("Error {} while retrieving data", res.status_code))
    }
}

macro_rules! return_status {
    ($status:literal) => {
        {
            println!("Status: {}\n", $status);
            return;
        }
    };
}

macro_rules! get_param {
    ($status:literal) => {
        {
            match std::env::args().filter(|a| a.starts_with($status)).nth(0) {
                Some(p) => match p[$status.len()..].parse() {
                    Ok(p) => p,
                    Err(_) => return_status!(400)
                },
                None => return_status!(400)
            }
        }
    };
}

fn main() {
    println!("Content-Type: text/html");

    // Get parameters from args (=requested date)
    let year = get_param!("year=");
    let month = get_param!("month=");
    let day = get_param!("day=");

    // Get Wordle HTML
    let str = match get(URL) {
        Ok(d) => d,
        Err(_) => return_status!(404)
    };

    // Extract JS bundle ID
    let re = Regex::new(r#"<script defer="" type="text/javascript" src="https://www.nytimes.com/games-assets/v2/wordle.(.*?).js""#).unwrap();
    let bundle_id = match re.captures(&str) {
        Some(c) => c,
        None => return_status!(500),
    };

    // Build URL for JavaScript and get it
    let url = format!("https://www.nytimes.com/games-assets/v2/wordle.{}.js", &bundle_id[1]);
    let str = match get(&url) {
        Ok(d) => d,
        Err(_) => return_status!(404)
    };

    // Find arrays with words. Note that there are two arrays. The longer one contains
    // all words that can be entered. The shorter one contains the daily results.
    let re = Regex::new(r#"\[(("\w{5}",?)+)\]"#).unwrap();
    let arrays = re.captures_iter(&str).collect::<Vec<Captures>>();
    match arrays.len() {
        0 => return_status!(404),
        2 => { },
        _ => return_status!(500),
    }

    let shorter_word_array = &arrays[if arrays[0].len() > arrays[1].len() { 0 } else { 1 }];
    let words = shorter_word_array[1]
        .split(',') // Split to get words
        .map(|s| &s[1..s.len() - 1]) // Remove quotes
        .collect::<Vec<&str>>();
    
    // Calculate word index and get word
    let start = Utc.ymd(2021, 6, 19); // Start of Wordle
    let date_to_find = Utc.ymd(year, month, day);
    let todays_offset = (((date_to_find - start).num_milliseconds() as f64) / 864e5).round() as usize;
    let word = words[todays_offset % words.len()];

    // Return HTML result
    println!();
    println!("<html><body>");
    println!("<p>The Wordle result for {year}-{month}-{day} is <b>{word}</b>. You're welcome!</p>");
    println!("<p>Go on and try it at <a href='https://www.nytimes.com/games/wordle/index.html'>https://www.nytimes.com/games/wordle/index.html</a>.</p>");
    println!("</body></html>");
}
