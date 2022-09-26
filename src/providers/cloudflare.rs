use core::fmt;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use reqwest::blocking::{Client, Response};
use reqwest::header::HeaderMap;
use serde_json::{from_str, Value};
use url::Url;

#[path = "../stats.rs"]
mod stats;

const BASE_URL: &str = "https://speed.cloudflare.com";
const ALL_CDN: &str = "locations";
const ACTIVE_CDN: &str = "cdn-cgi/trace";
const DOWNLOAD: &str = "__down";
const UPLOAD: &str = "__up";

struct ServerInfo {
    location: String,
    client_ip: String,
}

impl fmt::Display for ServerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Server Location:\t{location}\nYour IP:\t\t{ip}",
            location = self.location.replace('"', ""),
            ip = self.client_ip
        )
    }
}

struct LatencyInfo {
    minimum: Option<f32>,
    maximum: Option<f32>,
    average: Option<f32>,
    median: Option<f32>,
    jitter: Option<f32>,
}

impl fmt::Display for LatencyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Latency (avg/jitter):\t{avg:.2} ms / {jitter:.2} ms",
            avg = self.average.unwrap(),
            jitter = self.jitter.unwrap(),
        )
    }
}

struct SpeedInfo {
    minimum: Option<f32>,
    maximum: Option<f32>,
    average: Option<f32>,
}

impl fmt::Display for SpeedInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.average.unwrap())
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

fn parse_server_time_from_response(headers: HeaderMap) -> Duration {
    return Duration::from_micros(
        (headers
            .get("server-timing")
            .unwrap()
            .to_str()
            .unwrap()
            .split("=")
            .last()
            .unwrap()
            .parse::<f64>()
            .unwrap()
            * 1_000.0) as u64,
    );
}

fn download(nbytes: Option<isize>, nrepeat: Option<isize>) -> SpeedInfo {
    let mut xfer_times: Vec<Duration> = Vec::new();

    let client = Client::new();
    let request_url = Url::parse(BASE_URL).unwrap().join(DOWNLOAD).unwrap();
    let query = client
        .get(request_url.clone())
        .query(&[("bytes", nbytes.unwrap_or(0))]);

    for _ in 0..nrepeat.unwrap_or(1) {
        let start = Instant::now();
        let response = query.try_clone().unwrap().send();

        let unwrapped_response = response.unwrap();
        let header = unwrapped_response.headers().to_owned();

        unwrapped_response.bytes();

        let duration = start.elapsed();
        let server_delay = parse_server_time_from_response(header);

        if duration > server_delay {
            xfer_times.push(duration - server_delay);
        }
    }

    let mut speed: Vec<f32> = Vec::new();

    for xfer_time in &xfer_times {
        speed.push((nbytes.unwrap_or(0) as f32 * 8.0) / xfer_time.as_secs_f32() / 1.0e6)
    }

    return SpeedInfo {
        minimum: Some(stats::minimum(speed.clone())),
        maximum: Some(stats::maximum(speed.clone())),
        average: Some(stats::average(speed.clone())),
    };
}

fn measure_download() {
    println!(
        "100kB Download: \t{} Mbps",
        download(Some(100_000), Some(10))
    );
    println!(
        "1MB Download: \t\t{} Mbps",
        download(Some(1_000_000), Some(8))
    );
    println!(
        "10MB Download: \t\t{} Mbps",
        download(Some(10_000_000), Some(6))
    );
    println!(
        "25MB Download: \t\t{} Mbps",
        download(Some(25_000_000), Some(4))
    );
    println!(
        "100MB Download: \t{} Mbps",
        download(Some(100_000_000), Some(1))
    );
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

pub fn measure_latency(count: Option<u32>) {
    let mut latency: Vec<f32> = Vec::new();

    let client = Client::new();
    let request_url = Url::parse(BASE_URL).unwrap().join(DOWNLOAD).unwrap();
    let query = client.get(request_url.clone()).query(&[("bytes", "0")]);

    for _ in 0..count.unwrap_or(20) {
        let start = Instant::now();
        let response = query.try_clone().unwrap().send();
        let header = response.unwrap().headers().to_owned();

        let duration = start.elapsed();
        let server_delay = parse_server_time_from_response(header);

        if duration > server_delay {
            latency.push((duration - server_delay).as_secs_f32() * 1_000.0);
        }
    }

    let latency_info = LatencyInfo {
        minimum: Some(stats::minimum(latency.clone())),
        maximum: Some(stats::maximum(latency.clone())),
        average: Some(stats::average(latency.clone())),
        median: Some(stats::median(latency.clone())),
        jitter: Some(stats::jitter(latency.clone())),
    };

    println!("{}", latency_info);
}

pub fn measure_speed() {
    measure_download()
}
