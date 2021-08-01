use std::convert::TryFrom;
use std::path::PathBuf;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn load_dotenv() {
    dotenv::dotenv().ok();

    let env = dotenv::var("RUST_ENV").unwrap_or_else(|_| String::from("prod"));
    dotenv::from_filename(format!(".env.{}", env)).ok();
}

pub fn setup_tracing<P>(
    log_dir: Option<P>,
    log_file_prefix: P,
) -> crate::error::Result<tracing_appender::non_blocking::WorkerGuard>
where
    P: Into<PathBuf>,
{
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_from("info"))
        .unwrap();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(false)
        .compact();

    let (file_appender, guard) = tracing_appender::non_blocking(tracing_appender::rolling::daily(
        log_dir
            .map(|dir| dir.into())
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .as_path(),
        log_file_prefix.into().as_path(),
    ));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(false)
        .json()
        .with_writer(file_appender);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(file_layer)
        .try_init()?;

    Ok(guard)
}
