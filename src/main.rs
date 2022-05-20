extern crate argparse;

use regex::Regex;
use std::fs::File;
use std::collections::HashMap;
use std::io::{ BufReader, prelude::* };
use argparse::{ArgumentParser, StoreTrue, Store};

fn check_url(url: &str)-> bool {
    let re = Regex::new(r"http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*\(\),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+").unwrap();
    let _ret = re.is_match(url);
    return _ret;
}

async fn send_request(url: String, word: String) -> Result<(), Box<dyn std::error::Error>> {
    let final_url: String = url.replace("FUZZ", &word.trim());
    let resp = reqwest::get(final_url).await?.json::<HashMap<String, String>>().await?;
    println!("{:#?}", resp);
    Ok(())
}

async fn engine(url: String, wordlist: String, threads: u32){
    let file = File::open(wordlist).expect("Can't open file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut cnt: u32 = 0;
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("Total words : {cnt}");
                break;
            }
            Ok(_) => {
                cnt += 1;
                send_request(url.clone(), line.clone()).await;
                line.clear();
            }
            Err(err) => {
                continue;
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut verbose: bool = false;
    let mut url: String = "".to_string();
    let mut thread: u32 = 0;
    let mut wordlist: String = "".to_string();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Angmar 0.1 (https://github.com/naiame-neltheri/angmar)");
        parser.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue, "Verbose output");
        parser.refer(&mut url).add_option(&["-u", "--url"], Store, "URL to scrape").required();
        parser.refer(&mut wordlist).add_option(&["-w", "--wordlist"], Store, "Wordlist to use").required();
        parser.refer(&mut thread).add_option(&["-t", "--thread"], Store, "Number of threads, default 0");
        parser.parse_args_or_exit();
    }
    if !check_url(&url) {
        println!("Invalid url: {url}");
        std::process::exit(1);
    }
    engine(url, wordlist, thread).await;
    Ok(())
}