

use serde_derive::Deserialize;

// Define log levels as constants
const LOG_LEVEL_DEBUG: i32 = -1;
const LOG_LEVEL_INFO: i32 = 0;
const LOG_LEVEL_WARN: i32 = 1;
const LOG_LEVEL_ERROR: i32 = 2;
const LOG_LEVEL_NONE: i32 = 3;
//

// Function to log a message if the log level is enabled
pub fn log_message(level: i32, message: &str) {
  if is_log_enabled(level) {
    unsafe {
      log(level, message.as_ptr() as *const i32,  message.len() as i32);
    }
  }
}

// Function to check if logging is enabled for a given level
pub fn is_log_enabled(level: i32) -> bool {
    unsafe { log_enabled(level) != 0 }
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

extern "C" {
    fn get_config(buf: i32, buf_limit: i32) -> i32;
    fn log(level: i32, message: *const i32, message_len: i32);
    fn log_enabled(level: i32) -> i32;
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
pub extern "C" fn handle_request() -> u64 {
    // TODO: Implement logging, currently in trouble with i32 memory allocation
    // log_message(LOG_LEVEL_DEBUG, "DEADBEEF");

    // Example: Add a header and proceed to the next handler
    // directly and return 0 to skip the next handler.

    // For now, we proceed to the next handler with context
    16<<32|1 as u64
}

#[no_mangle]
pub extern "C" fn handle_response(_req_ctx: i32, _is_error: i32) {

}