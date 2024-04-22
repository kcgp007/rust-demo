use std::io;
use tracing::Level;
use tracing_appender::rolling;
use tracing_error::ErrorLayer;
use tracing_subscriber::fmt::time;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Registry};

pub fn init() {
    let lv = Level::DEBUG;
    // 输出到stdout
    let subscriber = fmt::layer()
        .with_timer(time::ChronoLocal::rfc_3339())
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(io::stdout.with_max_level(lv))
        .pretty();

    // 输出到文件
    let file_subscriber = fmt::layer()
        .with_ansi(false)
        .with_timer(time::ChronoLocal::rfc_3339())
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(rolling::daily("log", "demo.log"))
        .pretty();

    // 加入默认
    Registry::default()
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        .with(subscriber)
        .with(file_subscriber)
        .init();

    // 安裝 color-eyre 的 panic 处理句柄
    color_eyre::install().expect("color-eyre安装失败");
}
