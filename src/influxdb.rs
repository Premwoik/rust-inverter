extern crate reqwest;

use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Response;
use std::future::Future;

pub fn influx_new_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer fUqLk2fqqb62jyE2PYNpx1mNbu38s75SKN7thO1nKpNqf2vRzb24QWopAlUjh-WM54xJ2KJA2_jXDYzGSlPKDQ=="));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    return Client::builder().default_headers(headers).build().unwrap();
}

pub fn write_energy_counters(
    client: &Client,
    counters: &[(&str, f32)],
) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let url: &str = "http://192.168.2.100:8086/api/v2/write";
    let bucket: &str = "bucket=default";
    let org: &str = "org=SmartHome";

    let data: String = counters
        .iter()
        .copied()
        .map(|(c, v)| format!("energy_meter,meter_id={c} usage={v}\n"))
        .collect();
    let mut url = reqwest::Url::parse(url).unwrap();

    url.set_query(Some(org));
    url.set_query(Some(bucket));
    return client.post(url).body(data).send();
}
