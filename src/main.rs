mod logging;

use tch::nn::{Module, OptimizerConfig};
use tch::{nn, Device};
use tracing::{error, info, trace};

fn main() {
    logging::init();

    info!("aaa");

    trace!("error");

    error!("error");

    // run().unwrap();
}

// fn net(vs: &nn::Path) -> impl Module {
//     nn::seq()
//         .add(nn::linear(
//             vs / "layer1",
//             IMAGE_DIM,
//             HIDDEN_NODES,
//             Default::default(),
//         ))
//         .add_fn(|xs| xs.relu())
//         .add(nn::linear(vs, HIDDEN_NODES, LABELS, Default::default()))
// }

// fn run() -> Result<()> {
//     let m = tch::vision::mnist::load_dir("data")?;
//     let vs = nn::VarStore::new(Device::Cpu);
//     let net = net(&vs.root());
//     let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
//     for epoch in 1..200 {
//         let loss = net
//             .forward(&m.train_images)
//             .cross_entropy_for_logits(&m.test_labels);
//         opt.backward_step(&loss);
//         let test_accuracy = net
//             .forward(&m.test_images)
//             .accuracy_for_logits(&m.test_labels);
//         println!(
//             "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
//             epoch,
//             f64::from(&loss),
//             100. * f64::from(&test_accuracy),
//         );
//     }
//     Ok(())
// }
