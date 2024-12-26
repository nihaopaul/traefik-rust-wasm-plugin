use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareConfig {
    pub cf_domain: String,
    pub cf_org: String,
    pub cf_token: String,
}

use traefik_wasm_api;

// Returns the Structured Configuration from the host in JSON format
lazy_static! {
    static ref CONFIG: CloudflareConfig = {
        match serde_json::from_slice(&traefik_wasm_api::get_conf()) {
            Ok(config) => {
                traefik_wasm_api::send_log(
                    traefik_wasm_api::DEBUG,
                    format!("{:?}", config).as_str(),
                );
                config
            }
            Err(e) => {
                let empty_config = CloudflareConfig {
                    cf_domain: "NO_DOMAIN_DEFINED".to_string(),
                    cf_org: "NO_ORG_DEFINED".to_string(),
                    cf_token: "NO_TOKEN_DEFINED".to_string(),
                };
                traefik_wasm_api::send_log(traefik_wasm_api::WARN, format!("{:?}", e).as_str());
                traefik_wasm_api::send_log(
                    traefik_wasm_api::WARN,
                    format!("{:?}", empty_config).as_str(),
                );
                empty_config
            }
        }
    };
}

#[export_name = "handle_request"]
pub fn http_request() -> i64 {
    // let conf: &CloudflareConfig = &*CONFIG;
    // let headers = &traefik_wasm_api::get_headers(traefik_wasm_api::REQUEST_HEADER);

    // let header = "user-agent";
    // let header_values = &traefik_wasm_api::get_header_val(traefik_wasm_api::REQUEST_HEADER, &header);
    // traefik_wasm_api::send_log(traefik_wasm_api::DEBUG, format!("{:?}", header_values).as_str());

    // traefik_wasm_api::send_log(traefik_wasm_api::WARN, format!("{:?}", features).as_str());
    let items = traefik_wasm_api::get_headers(traefik_wasm_api::REQUEST_HEADER);

    for s in items {
        // each String is moved into s here...
        traefik_wasm_api::get_header_val(traefik_wasm_api::REQUEST_HEADER, s.as_str());
        traefik_wasm_api::send_log(traefik_wasm_api::DEBUG, format!("{:?}", s).as_str());
    } // ...an

    return 16 << 32 | 1 as i64;
}

#[export_name = "handle_response"]
fn http_response(_req_ctx: i32, _is_error: i32) {
    traefik_wasm_api::send_log(traefik_wasm_api::INFO, format!("{:?}", _req_ctx).as_str())
}

fn main() {}
