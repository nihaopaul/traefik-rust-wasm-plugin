#![allow(dead_code)]

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

#[link(wasm_import_module = "http_handler")]
extern "C" {
    fn log(level: i32, message: *const i32, message_len: i32);
    fn get_config(buf: *const i32, buf_limit: i32) -> i32;
    fn get_method(buf: *const i32, buf_limit: i32) -> i32;
    fn set_method(ptr: *const i32, message_len: i32);
    fn get_uri(ptr: *const i32, message_len: i32) -> i32;
    fn set_uri(ptr: *const i32, message_len: i32);
    fn get_protocol_version(ptr: *const i32, message_len: i32) -> i32;
    fn add_header_value(
        header_kind: i32,
        name_ptr: *const i32,
        name_len: i32,
        value_ptr: *const i32,
        value_len: i32,
    );
    fn set_header_value(
        header_kind: i32,
        name_ptr: *const i32,
        name_len: i32,
        value_ptr: *const i32,
        value_len: i32,
    );
    fn remove_header(header_kind: i32, name_ptr: *const i32, name_len: i32);
    fn get_header_names(header_kind: i32, buf: *const i32, buf_limit: i32) -> i64;
    // TODO: implement multiple header request
    fn get_header_values(
        header_kind: i32,
        name_ptr: *const i32,
        name_len: i32,
        buf: *const i32,
        buf_limit: i32,
    ) -> i64;
    fn log_enabled(level: i32) -> i32;
    fn read_body(body_kind: i32, ptr: *const i32, buf_limit: i32) -> i64;
    fn write_body(body_kind: i32, ptr: *const i32, message_len: i32);
    fn get_status_code() -> i32;
    fn set_status_code(code: i32);
    fn enable_features(feature: i32) -> i32;
    fn get_source_addr(buf: *const i32, buf_limit: i32) -> i32;
}

pub fn status_code() -> i32 {
    // ;; get_status_code returns the status code produced by the next handler defined
    // ;; on the host, e.g. 200.
    // ;;
    // ;; Note: A host who fails to get the status code will trap (aka panic,
    // ;; "unreachable" instruction).
    // (import "http_handler" "get_status_code" (func $get_status_code
    //   (result (; len ;) i32)))
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
    // ;; set_status_code overwrites the status code produced by the next handler defined
    // ;; on the host, e.g. 200. To call this in `handle_response` requires
    // ;;`feature_buffer_response`.
    // ;;
    // ;; Note: A host who fails to set the status code will trap (aka panic,
    // ;; "unreachable" instruction).
    // (import "http_handler" "set_status_code" (func $set_status_code
    //   (param $status_code i32)))

    unsafe { set_status_code(code) };
}

pub fn writebody(kind: i32, message: &str) {
    // ;; write_body reads `body_len` bytes at memory offset `body` and writes them to
    // ;; the pending body.
    // ;;
    // ;; Unlike `set_XXX` functions, this function is stateful, so repeated calls
    // ;; write to the current stream.
    // ;;
    // ;; Note: A host who fails to write the body will trap (aka panic, "unreachable"
    // ;; instruction).
    // (import "http_handler" "write_body" (func $write_body
    //   (param $kind i32)
    //   (param $body i32) (param $body_len i32)))
    unsafe { write_body(kind, message.as_ptr() as *const i32, message.len() as i32) };
}

pub fn is_log_enabled(level: i32) -> i32 {
    // ;; log_enabled returns 1 if the $level is enabled. This value may be cached
    // ;; at request granularity.
    // (import "http_handler" "log_enabled" (func $log_enabled
    //   (param $level i32)
    //   (result (; 0 or enabled(1) ;) i32)))
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

        let read_buf: Vec<u8> = vec![0u8; len as usize];

        match get_header_names(kind, read_buf.as_ptr() as *const i32, len) {
            len => {
                let data: &[u8] = &read_buf[0..len as usize];
                return str_array_from_u8_nul_utf8_unchecked(data);
            }
        }
    };
}

pub fn rem_header(kind: i32, name: &str) {
    // ;; remove_header removes all values for a header with the given name.
    // ;;
    // ;; Note: A host who fails to remove the header will trap (aka panic,
    // ;; "unreachable" instruction).
    // (import "http_handler" "remove_header" (func $set_header_value
    //   (param  $kind i32)
    //   (param  $name i32) (param $name_len i32)))

    unsafe { remove_header(kind, name.as_ptr() as *const i32, name.len() as i32) };
}

pub fn set_header(kind: i32, name: &str, value: &str) {
    // ;; set_header_value overwrites all values of the given header name with the
    // ;; input.
    // ;;
    // ;; Note: A host who fails to set the header will trap (aka panic,
    // ;; "unreachable" instruction).
    // (import "http_handler" "set_header_value" (func $set_header_value
    //   (param  $kind i32)
    //   (param  $name i32) (param $name_len i32)
    //   (param $value i32) (param $value_len i32)))
    unsafe {
        set_header_value(
            kind,
            name.as_ptr() as *const i32,
            name.len() as i32,
            value.as_ptr() as *const i32,
            value.len() as i32,
        )
    };
}

pub fn add_header(kind: i32, name: &str, value: &str) {
    // ;; add_header_value adds a single value for the given header name.
    // ;;
    // ;; Note: A host who fails to add the header will trap (aka panic,
    // ;; "unreachable" instruction).
    // (import "http_handler" "add_header_value" (func $add_header_value
    //   (param  $kind i32)
    //   (param  $name i32) (param $name_len i32)
    //   (param $value i32) (param $value_len i32)))

    unsafe {
        add_header_value(
            kind,
            name.as_ptr() as *const i32,
            name.len() as i32,
            value.as_ptr() as *const i32,
            value.len() as i32,
        )
    };
}

pub fn send_log(level: i32, message: &str) {
    // ;; log adds a UTF-8 encoded message to the host's logs at the given $level.
    // ;;
    // ;; Note: A host who fails to log a message should ignore it instead of a trap
    // ;; (aka panic, "unreachable" instruction).
    // (import "http_handler" "log" (func $log
    //   (param $level (; log_level ;) i32)
    //   (param $message i32) (param $message_len i32)))

    unsafe { log(level, message.as_ptr() as *const i32, message.len() as i32) };
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
    // ;; set_method overwrites the method with one read from memory, e.g. "POST".
    // ;;
    // ;; Note: A host who fails to set the method will trap (aka panic, "unreachable"
    // ;; instruction).
    // (import "http_handler" "set_method" (func $set_method
    //   (param $method i32) (param $method_len i32)))
    unsafe { set_method(method.as_ptr() as *const i32, method.len() as i32) };
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
    // ;; set_uri overwrites the URI with one read from memory, e.g.
    // ;; "/v1.0/hi?name=panda".
    // ;;
    // ;; Note: The URI may include query parameters. The guest MUST pass
    // ;; the URI encoded as the host will ALWAYS expect the URI as encoded
    // ;; and passing it unencoded could lead to unexpected behaviours.
    // ;;
    // ;; Note: A host who fails to set the URI will trap (aka panic, "unreachable"
    // ;; instruction).
    // (import "http_handler" "set_uri" (func $set_uri
    //   (param $uri i32) (param $uri_len i32)))
    unsafe { set_uri(uri.as_ptr() as *const i32, uri.len() as i32) };
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
