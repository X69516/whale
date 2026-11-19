mod bootstrap;
mod shutdown;

use std::process::ExitCode;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> ExitCode {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    if let Err(error) = tracing_subscriber::fmt().with_env_filter(filter).try_init() {
        eprintln!("serverd: logging initialization failed: {error}");
        return ExitCode::FAILURE;
    }

    match bootstrap::run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!(
                error = %error,
                "Server startup or runtime failed"
            );

            ExitCode::FAILURE
        }
    }
}
