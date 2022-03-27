use std::error::Error;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use serde_json::{json, Map, Value};
use vercel_lambda::{Body, Handler, IntoResponse, Request, Response};
use vercel_lambda::error::VercelError;
use vercel_lambda::http::StatusCode;

use crate::ASNs;

pub struct VercelAsnHandler {
    pub asns_arc: Arc<RwLock<Arc<ASNs>>>,
}

impl VercelAsnHandler {
    fn ip_lookup(&self, ip_str: &str) -> Result<serde_json::Map<String, Value>, Box<dyn Error>> {
        let ip = IpAddr::from_str(ip_str).or_else(|e| Err(format!("Invalid ip address: {}", e)))?;
        let asns = self.asns_arc.read().unwrap().clone();
        let mut map = serde_json::Map::new();
        map.insert(
            "ip".to_string(),
            serde_json::value::Value::String(ip_str.to_string()),
        );
        let found = match asns.lookup_by_ip(ip) {
            None => {
                map.insert(
                    "announced".to_string(),
                    serde_json::value::Value::Bool(false),
                );
                return Ok(map);
            }
            Some(found) => found,
        };
        map.insert(
            "announced".to_string(),
            serde_json::value::Value::Bool(true),
        );
        map.insert(
            "first_ip".to_string(),
            serde_json::value::Value::String(found.first_ip.to_string()),
        );
        map.insert(
            "last_ip".to_string(),
            serde_json::value::Value::String(found.last_ip.to_string()),
        );
        map.insert(
            "as_number".to_string(),
            serde_json::value::Value::Number(serde_json::Number::from(found.number)),
        );
        map.insert(
            "as_country_code".to_string(),
            serde_json::value::Value::String(found.country.clone()),
        );
        map.insert(
            "as_description".to_string(),
            serde_json::value::Value::String(found.description.clone()),
        );
        Ok(map)
    }
}

impl Handler<Response<String>, Body, VercelError> for VercelAsnHandler {
    fn run(&mut self, req: vercel_lambda::http::Request<Body>) -> Result<Response<String>, VercelError> {
        let uri = req.uri().path();
        let ip = uri.rsplit('/').next().unwrap_or("");
        match self.ip_lookup(ip) {
            Ok(map) => {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&map).unwrap())
                    .expect("Internal Server Error"))
            }
            Err(e) => {
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .body(json!({ "error": format!("{}", e) }).to_string())
                    .expect("Internal Server Error"))
            }
        }
    }
}
