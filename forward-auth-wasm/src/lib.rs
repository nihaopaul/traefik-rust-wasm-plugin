
use log::{info};



#[no_mangle]
pub extern "C" fn handle_request() -> u64 {
  info!("Reequest: ");
  return 0 as u64;
}


#[no_mangle]
pub extern "C" fn handle_response(high:i32, low: i32)  {
  let res = (high as u64) << 32 | low as u64;
  info!("Response: {res}");
}