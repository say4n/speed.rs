use core::fmt;
use std::collections::HashMap;

use reqwest::blocking::Client;
use serde_json::{from_str, Value};
use url::Url;

const BASE_URL: &str = "https://speed.cloudflare.com";
const ALL_CDN: &str = "locations";
const ACTIVE_CDN: &str = "cdn-cgi/trace";

pub struct ServerInfo {
    location: String,
    client_ip: String,
}

impl fmt::Display for ServerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Server Location: \t{location}\nYour IP: \t\t{ip}",
            location = self.location.replace('"', ""),
            ip = self.client_ip
        )
    }
}

fn fetch_all_cdn_info() -> Value {
    let request_url = Url::parse(BASE_URL).unwrap().join(ALL_CDN).unwrap();

    let client = Client::new();
    let response = client.get(request_url).send();
    let raw_text = response.unwrap().text().unwrap();
    let parsed_json: Value = from_str(&raw_text).unwrap();

    // println!("{:#}", parsed_json);

    return parsed_json;
}

fn fetch_active_cdn_info() -> HashMap<String, String> {
    let request_url = Url::parse(BASE_URL).unwrap().join(ACTIVE_CDN).unwrap();

    let client = Client::new();
    let response = client.get(request_url).send();
    let raw_text = response.unwrap().text().unwrap();

    let lines = raw_text.lines();
    let mut data: HashMap<String, String> = HashMap::new();

    for l in lines {
        let kv: Vec<&str> = l.split("=").collect();
        data.insert(kv[0].to_string(), kv[1].to_string());
    }

    // println!("{:?}", data);

    return data;
}

pub fn get_server_info() {
    let all_cdn_data = fetch_all_cdn_info();
    let active_cdn_data = fetch_active_cdn_info();

    let active_cdn_details: Vec<&Value> = all_cdn_data
        .as_array()
        .unwrap()
        .iter()
        .filter(|x| {
            x.as_object().unwrap().get("iata").unwrap().as_str()
                == Some(active_cdn_data.get("colo").unwrap().as_str())
        })
        .collect();
    let active_cdn_details_for_real = active_cdn_details.get(0).unwrap().as_object().unwrap();

    let server_info = ServerInfo {
        location: format!(
            "{city} ({iata})",
            city = active_cdn_details_for_real.get("city").unwrap(),
            iata = active_cdn_details_for_real.get("iata").unwrap(),
        ),
        client_ip: format!(
            "{ip} ({region})",
            ip = active_cdn_data.get("ip").unwrap(),
            region = active_cdn_data.get("loc").unwrap(),
        ),
    };

    println!("{}", server_info);
}
