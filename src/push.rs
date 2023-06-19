use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::data::SensorDataValues;


pub fn send(data: SensorDataValues, url: String, id: &str) {
    let mut headers = HeaderMap::new();
    headers.insert("Sensor", HeaderValue::from_str(id).unwrap());
    let client = Client::new();
    let resp = client.post(url).headers(headers).json(&data).send();

    println!("body = {:?}", resp.unwrap().text());
}
