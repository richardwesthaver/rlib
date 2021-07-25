use crate::log::{debug, info, trace, warn};
use crate::simple;
#[test]
fn simple_init() {
  let log = simple();
  info!("now this is podracing");
  debug!("a debug message");
  warn!("gotchya!");
  trace!("hisssss");
}
