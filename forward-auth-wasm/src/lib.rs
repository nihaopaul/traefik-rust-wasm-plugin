use guest::CloudflareConfig;
use lazy_static::lazy_static;
mod guest;

// Returns the Structured Configuration from the host in JSON format
lazy_static! {
    static ref CONFIG: CloudflareConfig = {
        match serde_json::from_slice(&guest::get_conf()) {
            Ok(config) => {
                guest::send_log(guest::DEBUG, format!("{:?}", config).as_str());
                config
            }
            Err(e) => {
                let empty_config = CloudflareConfig {
                    cf_domain: "NO_DOMAIN_DEFINED".to_string(),
                    cf_org: "NO_ORG_DEFINED".to_string(),
                    cf_token: "NO_TOKEN_DEFINED".to_string(),
                };
                guest::send_log(guest::WARN, format!("{:?}", e).as_str());
                guest::send_log(guest::WARN, format!("{:?}", empty_config).as_str());
                empty_config
            }
        }
    };
}

#[export_name = "handle_request"]
pub fn http_request() -> i64 {
    // let conf: &CloudflareConfig = &*CONFIG;
    // let headers = &guest::get_headers(guest::REQUEST_HEADER);

    // let header = "user-agent";
    // let header_values = &guest::get_header_val(guest::REQUEST_HEADER, &header);
    // guest::send_log(guest::DEBUG, format!("{:?}", header_values).as_str());

    let method = &guest::get_request_method();
    guest::send_log(guest::DEBUG, format!("{:?}", method).as_str());

    return 16 << 32 | 1 as i64;
}

#[export_name = "handle_response"]
fn http_response(_req_ctx: i32, _is_error: i32) {
    guest::send_log(guest::INFO, format!("{:?}", _req_ctx).as_str())
}
