mod demo;
mod logging;

fn main() {
    logging::init();

    // info!("aaa");
    // trace!("error");
    // error!("error");

    demo::demo7();
}
