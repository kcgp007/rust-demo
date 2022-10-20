mod logging;
mod tch_demo;
mod tf_demo;

use tch::nn::{Module, OptimizerConfig};
use tch::{nn, Device};
use tracing::{error, info, trace};

fn main() {
    logging::init();

    info!("aaa");

    trace!("error");

    error!("error");

    tch_demo::run().unwrap();
}
