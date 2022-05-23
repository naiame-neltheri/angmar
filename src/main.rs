extern crate argparse;

use regex::Regex;
use std::fs::File;
use std::io::{ BufReader, prelude::* };
use argparse::{ArgumentParser, StoreTrue, Store};

static mut VERBOSE: bool = false;
static mut FOUND: Vec<ResponseData> = Vec::new();

struct ResponseData {
    status_code: u16,
    length: Option<u64>,
    version: reqwest::Version,
    request_url: String
}

fn check_url(url: &str)-> bool {
    let re = Regex::new(r"http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*\(\),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+").unwrap();
    let _ret = re.is_match(url);
    return _ret;
}

fn parse_output(resp: ResponseData) {
    print!("\r{0:<25} | {1:<11} | {2:<10?}", resp.request_url, resp.status_code, resp.version);
    unsafe {
        if (FOUND.len() != 0) {
            for elem in FOUND.iter() {
                println!("\n\n{0:<25} | {1:<11} | {2:<10?}", elem.request_url, elem.status_code, elem.version);
            }
        }
        if (resp.status_code == 200) {
            FOUND.push(resp)
        }
    }
}

async fn send_request(url: String, word: String) -> Result<(), Box<dyn std::error::Error>> {
    let final_url: String = url.replace("FUZZ", &word.trim());
    let client = reqwest::Client::builder().build()?;
    let resp = client.get(final_url.clone()).send().await.expect("Cannot make HTTP request: ");
    let resp_data = ResponseData{
        status_code: resp.status().as_u16(),
        length: resp.content_length(),
        version: resp.version(),
        request_url: final_url
    };
    parse_output(resp_data);
    Ok(())
}

async fn engine(url: String, wordlist: String, threads: u32){
    let file = File::open(wordlist).expect("Can't open file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut cnt: u32 = 0;
    println!("{0:<25} | {1:<1} | {2:<10}", "URL Path", "Status Code", "Version");
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => {
                // println!("Total words : {cnt}");
                break;
            }
            Ok(_) => {
                cnt += 1;
                send_request(url.clone(), line.clone()).await;
                line.clear();
            }
            Err(err) => {
                println!("Error occuerd: {err}");
                continue;
            }
        }
    }
    println!("Total words tried : {cnt}");
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
    unsafe {
        VERBOSE = verbose;
    }
    if !check_url(&url) {
        println!("Invalid url: {url}");
        std::process::exit(1);
    }
    engine(url, wordlist, thread).await;
    Ok(())
}