mod logging;

use tracing::{error, info, trace};

fn main() {
    logging::init();

    info!("aaa");

    trace!("error");

    error!("error");
}
