extern crate reqwest;
extern crate clap;
extern crate log;
extern crate simple_logger;

use reqwest::Url;
use std::time::Duration;
use clap::{Arg, App, ArgMatches};
use log::{info, warn, error, Level};

struct RequestArguments {
    url: String,
    timeout: u64,
}

fn main() {
    let matches = parse_arguments();
    let m_res = match fetch_parsed_arguments(&matches) {
        Ok(res) => res,
        Err(e) => {
            error!("Error parsing arguments: {}", e);
            return;
        }
    };

    perform_request(&m_res.url, m_res.timeout);
}

fn parse_arguments() -> ArgMatches<'static> {
    let matches = App::new("health-checker")
        .version("1.0")
        .author("Stefan Szasz <stefanszasz2@gmail.com>")
        .about("Performs http service availability checking")
        .arg(Arg::with_name("url")
            .help("Make the request to this URL")
            .required(true)
            .index(1))
        .arg(Arg::with_name("timeout")
            .short("t")
            .default_value("10")
            .help("Sets default http client timeout"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .help("Sets verbosity of logging"))
        .get_matches();

    if matches.index_of("verbose").is_some() {
        simple_logger::init_with_level(Level::Debug);
    } else {
        simple_logger::init_with_level(Level::Info);
    }

    matches
}

fn fetch_parsed_arguments(matches: &ArgMatches) -> Result<RequestArguments, String> {
    let url = String::from(matches.value_of("url").expect("Url must be set"));
    let timeout_val = matches.value_of("timeout").unwrap();
    let timeout_res = timeout_val.parse::<u64>();
    let timeout: u64 = match timeout_res {
        Ok(res) => res,
        Err(_) => {
            let err = format!("Timeout must be expressed in seconds");
            return Err(err)
        }
    };

    let result = RequestArguments { timeout, url };

    Ok(result)
}

fn perform_request(url: &str, timeout: u64) {
    if !&url[..].starts_with("http") {
        error!("Url {} must start with http/https", url);
        return;
    }
    let uri: Url = url.parse().expect("Url is not in correct format");

    info!("Making request to \"{}\" with {} seconds timeout...", url, timeout);
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(timeout))
        .build().unwrap();

    let response = client.get(uri).send();
    match response {
        Ok(mut res) => if res.status().as_u16() >= 400 {
            let status = res.status();
            let outcome = status.as_str();
            error!("Failed response: {:?}", outcome);
            std::process::exit(1);
        } else {
            let text_res = match res.text() {
                Ok(r) => r,
                Err(e) => {
                    error!("Cannot fetch result text: {:?}", e);
                    return;
                }
            };
            let sub_text = &text_res[0..128];
            info!("Done with response:\n {}...", sub_text);
        },
        Err(e) => {
            error!("Got error: {}", e);
            std::process::exit(1);
        }
    };
}