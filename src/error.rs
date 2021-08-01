use snafu::{Backtrace, Snafu};
use tracing_subscriber::util::TryInitError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to initialize tracing: {}", source), context(false))]
    TracingInit {
        source: TryInitError,
        backtrace: Backtrace,
    },

    #[snafu(context(false))]
    Io {
        source: std::io::Error,
        backtrace: Backtrace,
    },
}
