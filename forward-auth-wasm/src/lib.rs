use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::str;

mod guest;

// TODO: this function does not quite return the Config struct yet but the bytes of decimals do
lazy_static! {
    static ref CONFIG: Config = {
        match serde_json::from_slice(&guest::get_conf()) {
            Ok(config) => config,
            Err(e) => {
                guest::send_log(guest::WARN, format!("{:?}", e).as_str());
                guest::send_log(guest::WARN, format!("{:?}", &guest::get_conf()).as_str());
                Config {
                    CF_DOMAIN: "NO_DOMAIN".to_string(),
                    CF_ORG: "NO_ORG".to_string(),
                    CF_TOKEN: "NO_TOKEN".to_string(),
                }
            }
        }
    };
}

fn main() {}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    CF_DOMAIN: String,
    CF_ORG: String,
    CF_TOKEN: String,
}

#[export_name = "handle_request"]
pub fn http_request() -> i64 {
    let conf: &Config = &*CONFIG;
    // for (k, v) in &conf.headers {
    //     guest::add_header(guest::REQUEST_HEADER, &k, &v);
    // }
    // guest::send_log(guest::DEBUG, format!("{:?}", guest::get_addr()).as_str());
    guest::send_log(guest::DEBUG, format!("{:?}", conf).as_str());
    return 16 << 32 | 1 as i64;
}

#[export_name = "handle_response"]
fn http_response(_req_ctx: i32, _is_error: i32) {}
