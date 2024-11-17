
use serde_derive::Deserialize;
use std::ffi::CString;


extern "C" {
  fn get_config(buf: i32, buf_limit: i32) -> i32;
  fn log(level: i32,message: i32, len: i32);
  fn log_enabled(level: i32) -> i32;
}

// Define log levels as constants
const LOG_LEVEL_DEBUG: i32 = -1;
const LOG_LEVEL_INFO: i32 = 0;
const LOG_LEVEL_WARN: i32 = 1;
const LOG_LEVEL_ERROR: i32 = 2;
const LOG_LEVEL_NONE: i32 = 3;
//



// Function to log a message if the log level is enabled
#[no_mangle]
pub fn log_message(level: i32, str: &str) {
  let _str = CString::new(str).unwrap();
  let bytes = _str.as_bytes();

  let len = bytes.len() as i32;
  unsafe {
    let ptr = bytes.as_ptr() as i32;
    log(level, ptr, len);
  }
}



// Function to call get_config and retrieve the configuration as a String
pub fn retrieve_config() -> Option<String> {
    // Allocate a buffer
    let mut buffer = vec![0u8; 1024];
    let buf_ptr = buffer.as_mut_ptr() as i32;
    let buf_limit = buffer.len() as i32;

    // Call the get_config function
    let len = unsafe { get_config(buf_ptr, buf_limit) };

    // Check if the length is valid
    if len > 0 && (len as usize) <= buffer.len() {
        // Convert the buffer to a String
        let config_str = String::from_utf8_lossy(&buffer[..len as usize]).to_string();
        Some(config_str)
    } else {
        None
    }
}




#[repr(C)]
#[derive(Deserialize)]
struct Request {
    method: String,
    path: String,
    headers: Vec<String>,
    source_addr: String,
    protocol_version: String,
}

#[repr(C)]
struct Response {
  headers: Vec<String>,
}

#[no_mangle]
pub extern fn handle_request() -> i64 {
  unsafe {
    let config: i32=  log_enabled(LOG_LEVEL_DEBUG);
  }
    // TODO: Implement logging, currently in trouble with i32 memory allocation
    // log_message(LOG_LEVEL_DEBUG, "DEADBEEF");

    // Example: Add a header and proceed to the next handler
    // directly and return 0 to skip the next handler.

    // For now, we proceed to the next handler with context
    16<<32|1
}

#[no_mangle]
pub extern fn handle_response(_req_ctx: i32, _is_error: i32) {
  // log_message(LOG_LEVEL_DEBUG, "DEADBEEF");
}

pub fn main() {
    // Retrieve the configuration
    let config = retrieve_config();
    if let Some(config_str) = config {
        log_message(LOG_LEVEL_INFO, &format!("Configuration: {}", config_str));
    } else {
        log_message(LOG_LEVEL_ERROR, "Failed to retrieve configuration");
    }
}