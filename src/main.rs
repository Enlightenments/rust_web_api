#[macro_use]
extern crate log;
extern crate env_logger;

mod app;

fn init_log() {
    use chrono::Local;
    use std::io::Write;
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env).format(|buf, record|
        { writeln!(buf, "{} {} [{}] {}",
                   Local::now().format("%Y-%m-%d %H:%M:%S"),
                   record.level(),
                   record.module_path().unwrap_or("<unnamed>"),
                   &record.args()) }).init();
    info!("env_logger initialized.");
}

#[tokio::main]
async fn main() {
    init_log();
    app::run(([127, 0, 0, 1],9999).into()).await;
}