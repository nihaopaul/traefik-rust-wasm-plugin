mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn handle_request(s: &str);
// }

#[wasm_bindgen]
pub fn handle_request() -> u64 {
  std::process::exit(1);
  // return 0 as u64;
}

