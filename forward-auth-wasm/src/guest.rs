// reference code from https://github.com/elisasre/http-wasm-rust/blob/main/src/guest.rs
use serde::{Deserialize, Serialize};
use std::{str, vec};

pub const FATAL: i32 = 3;
pub const ERROR: i32 = 2;
pub const WARN: i32 = 1;
pub const INFO: i32 = 0;
pub const DEBUG: i32 = -1;

pub const REQUEST_HEADER: i32 = 0;
pub const RESPONSE_HEADER: i32 = 1;
pub const REQUEST_HEADER_TRAILERS: i32 = 2;
pub const RESPONSE_HEADER_TRAILERS: i32 = 3;

pub const REQUEST_BODY: i32 = 0;
pub const RESPONSE_BODY: i32 = 1;

pub const FEATURE_BUFFER_REQUEST: i32 = 1;
pub const FEATURE_BUFFER_RESPONSE: i32 = 2;
pub const FEATURE_TRAILERS: i32 = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareConfig {
    pub cf_domain: String,
    pub cf_org: String,
    pub cf_token: String,
}

#[link(wasm_import_module = "http_handler")]
extern "C" {
    // working with log
    fn log(level: i32, message: *const u8, message_len: u32);
    // working with get_config
    fn get_config(buf: *const i32, buf_limit: i32) -> i32;
    // working with get_method
    fn get_method(buf: *const i32, buf_limit: i32) -> i32;
    // TODO: implement
    fn set_method(ptr: *const u8, message_len: u32);
    // working with get_uri
    fn get_uri(ptr: *const i32, message_len: i32) -> i32;
    // TODO: implement
    fn set_uri(ptr: *const u8, message_len: u32);
    // working get_protocol_version
    fn get_protocol_version(ptr: *const i32, message_len: i32) -> i32;
    // TODO: implement
    fn add_header_value(
        header_kind: u32,
        name_ptr: *const u8,
        name_len: u32,
        value_ptr: *const u8,
        value_len: u32,
    );
    // TODO: implement
    fn set_header_value(
        header_kind: u32,
        name_ptr: *const u8,
        name_len: u32,
        value_ptr: *const u8,
        value_len: u32,
    );
    // TODO: implement
    fn remove_header(header_kind: u32, name_ptr: *const u8, name_len: u32);

    // updated get_header_names signature
    fn get_header_names(header_kind: i32, buf: *const i32, buf_limit: i32) -> i64;

    // Done: get a header value
    // TODO: implement multiple header request
    fn get_header_values(
        header_kind: i32,
        name_ptr: *const i32,
        name_len: i32,
        buf: *const i32,
        buf_limit: i32,
    ) -> i64;

    // TODO: implement
    fn log_enabled(level: i32) -> i32;

    // read_body working
    fn read_body(body_kind: i32, ptr: *const i32, buf_limit: i32) -> i64;

    // TODO: implement
    fn write_body(body_kind: u32, ptr: *const u8, message_len: u32);

    // get_status_code working
    fn get_status_code() -> i32;
    // TODO: implement
    fn set_status_code(code: i32);
    // working with enable_features
    fn enable_features(feature: i32) -> i32;
    // working with get source address
    fn get_source_addr(buf: *const i32, buf_limit: i32) -> i32;
}

pub fn status_code() -> i32 {
    // (import "http_handler" "get_status_code" (func $get_status_code
    //   (result (; len ;) i32)))
    // Note: will panic if called before handle_response!
    unsafe { return get_status_code() };
}

pub fn enable_feature(feature: i32) -> i32 {
    // ;; enable_features tries to enable the given features and returns the entire
    // ;; feature bitflag supported by the host.
    // (import "http_handler" "enable_features" (func $enable_features
    //   (param $enable_features i32)
    //   (result  (; features ;) i32)))
    unsafe {
        match enable_features(feature) {
            res => {
                return res;
            }
        }
    };
}

pub fn readbody(kind: i32) -> Vec<u8> {
    // (import "http_handler" "read_body" (func $read_body
    //   (param $kind i32)
    //   (param  $buf i32) (param $buf_len i32)
    //   (result (; 0 or EOF(1) << 32 | len ;) i64)))
    let max_buffer_size = 1024 * 1024; // 1 MB buffer
    let read_buf = vec![0; max_buffer_size];

    unsafe {
        let mut eof = 1;
        let mut full_body = Vec::new();

        while eof != 0 {
            let response = read_body(
                kind,
                read_buf.as_ptr() as *const i32,
                max_buffer_size as i32,
            );
            eof = (response << 32) as i32;
            let len = response as i32;

            full_body.extend_from_slice(&read_buf[..len as usize]);
        }

        return full_body;
    };
}

pub fn set_code(code: i32) {
    unsafe { set_status_code(code) };
}

pub fn writebody(kind: u32, message: &str) {
    unsafe { write_body(kind, message.as_ptr(), message.len() as u32) };
}

pub fn log_enabl(level: i32) -> i32 {
    unsafe {
        match log_enabled(level) {
            res => return res,
        }
    };
}

// TODO: Implment multiple headers &[&str].
pub fn get_header_val(kind: i32, header: &str) -> Vec<String> {
    // ;; get_header_values writes all values of the given name, NUL-terminated, to
    // ;; memory if the encoded length isn't larger than `buf_limit`. `count_len` is
    // ;; returned regardless of whether memory was written. The name must be treated
    // ;; case-insensitive.
    // ;;
    // ;; Note: A host who fails to get header values will trap (aka panic,
    // ;; "unreachable" instruction).
    // (import "http_handler" "get_header_values" (func $get_header_values
    //   (param $kind i32)
    //   (param $name i32) (param  $name_len i32)
    //   (param  $buf i32) (param $buf_limit i32)
    //   (result (; count << 32| len ;) i64)))

    let name_ptr = header.as_ptr() as *const i32;
    let name_len = header.len() as i32;

    unsafe {
        let result = get_header_values(kind, name_ptr, name_len, std::ptr::null(), 0);

        let _count = (result >> 32) as i32;
        let len = result as i32;

        if len == 0 {
            return Vec::new();
        }

        let read_buf = vec![0u8; len as usize];

        match get_header_values(
            kind,
            name_ptr,
            name_len,
            read_buf.as_ptr() as *const i32,
            len,
        ) {
            len => {
                let data: &[u8] = &read_buf[0..len as usize];
                return str_array_from_u8_nul_utf8_unchecked(data);
            }
        }
    };
}

pub fn get_headers(kind: i32) -> Vec<String> {
    /*
    ;; get_header_names writes all header names, in lowercase, NUL-terminated, to
    ;; memory if the encoded length isn't larger than `buf_limit`. `count_len` is
    ;; returned regardless of whether memory was written.
    ;;
    ;; Note: A host who fails to get header names will trap (aka panic,
    ;; "unreachable" instruction).
    (import "http_handler" "get_header_names" (func $get_header_names
      (param $kind i32)
      (param  $buf i32) (param $buf_limit i32)
      (result (; count << 32| len ;) i64)))
       */
    unsafe {
        let result = get_header_names(kind, std::ptr::null(), 0);

        let _count = (result >> 32) as i32;
        let len = result as i32;

        if len == 0 {
            return Vec::new();
        }

        let read_buf = vec![0u8; len as usize];

        match get_header_names(kind, read_buf.as_ptr() as *const i32, len) {
            len => {
                let data: &[u8] = &read_buf[0..len as usize];
                return str_array_from_u8_nul_utf8_unchecked(data);
            }
        }
    };
}

pub fn rem_header(kind: u32, name: &str) {
    unsafe { remove_header(kind, name.as_ptr(), name.len() as u32) };
}

pub fn set_header(kind: u32, name: &str, value: &str) {
    unsafe {
        set_header_value(
            kind,
            name.as_ptr(),
            name.len() as u32,
            value.as_ptr(),
            value.len() as u32,
        )
    };
}

pub fn add_header(kind: u32, name: &str, value: &str) {
    unsafe {
        add_header_value(
            kind,
            name.as_ptr(),
            name.len() as u32,
            value.as_ptr(),
            value.len() as u32,
        )
    };
}

pub fn send_log(level: i32, message: &str) {
    unsafe { log(level, message.as_ptr(), message.len() as u32) };
}

pub fn get_conf() -> Vec<u8> {
    unsafe {
        // Get required buffer size
        let config_len = get_config(std::ptr::null(), 0);

        if config_len == 0 {
            return Vec::new();
        }

        // Allocate buffer
        let mut buffer = vec![0u8; config_len as usize];

        // Get actual config data
        match get_config(buffer.as_ptr() as *const i32, config_len) {
            len => {
                // Resize buffer to actual length and return
                buffer.truncate(len as usize);
                buffer
            }
        }
    }
}

pub fn get_addr() -> String {
    // (import "http_handler" "get_source_addr" (func $get_source_addr
    //   (param $buf i32) (param $buf_limit i32)
    //   (result (; len ;) i32)))
    unsafe {
        let result = get_source_addr(std::ptr::null(), 0);

        let len = result;

        if len == 0 {
            return String::new();
        }

        let read_buf = vec![0u8; len as usize];
        match get_source_addr(read_buf.as_ptr() as *const i32, len) {
            len => {
                return str::from_utf8(&read_buf[0..len as usize])
                    .unwrap()
                    .to_string();
            }
        }
    };
}

pub fn get_request_method() -> String {
    // (import "http_handler" "get_method" (func $get_method
    //   (param $buf i32) (param $buf_limit i32)
    //   (result (; len ;) i32)))

    unsafe {
        let result = get_method(std::ptr::null(), 0);

        let len = result;

        if len == 0 {
            return String::new();
        }

        let read_buf = vec![0u8; len as usize];

        match get_method(read_buf.as_ptr() as *const i32, 2048) {
            len => {
                return str::from_utf8(&read_buf[0..len as usize])
                    .unwrap()
                    .to_string();
            }
        }
    };
}

pub fn set_request_method(method: &str) {
    unsafe { set_method(method.as_ptr(), method.len() as u32) };
}

pub fn get_request_uri() -> String {
    // (import "http_handler" "get_uri" (func $get_uri
    //   (param $buf i32) (param $buf_limit i32)
    //   (result (; len ;) i32)))
    unsafe {
        let result = get_uri(std::ptr::null(), 0);

        let len = result;

        if len == 0 {
            return String::new();
        }

        let read_buf = vec![0u8; len as usize];

        match get_uri(read_buf.as_ptr() as *const i32, len) {
            response => {
                return str::from_utf8(&read_buf[0..response as usize])
                    .unwrap()
                    .to_string();
            }
        }
    };
}

pub fn set_request_uri(uri: &str) {
    unsafe { set_uri(uri.as_ptr(), uri.len() as u32) };
}

pub fn get_request_protocol_version() -> String {
    // (import "http_handler" "get_protocol_version" (func $get_protocol_version
    //   (param $buf i32) (param $buf_limit i32)
    //   (result (; len ;) i32)))
    unsafe {
        let result = get_protocol_version(std::ptr::null(), 0);

        let len = result;

        if len == 0 {
            return String::new();
        }

        let read_buf = vec![0u8; len as usize];

        match get_protocol_version(read_buf.as_ptr() as *const i32, len) {
            response => {
                return str::from_utf8(&read_buf[0..response as usize])
                    .unwrap()
                    .to_string();
            }
        }
    };
}

unsafe fn str_array_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut start_index: usize = 0;
    for (i, n) in utf8_src.iter().enumerate() {
        if *n == b'\0' {
            out.push(::std::str::from_utf8_unchecked(&utf8_src[start_index..i]).to_string());
            start_index = i + 1; // skip NUL byte
        }
    }
    return out;
}
