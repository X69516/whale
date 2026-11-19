pub async fn run() -> anyhow::Result<()> {
    tracing::info!("Whale Supervisor starting");

    tracing::info!("Initializing worker registry");
    tracing::info!("Initializing lifecycle manager");

    tracing::info!("Whale Supervisor ready");

    crate::shutdown::wait_for_signal().await?;

    tracing::info!("Shutdown requested");
    tracing::info!("Stopping managed workers");

    tracing::info!("Whale Supervisor stopped");

    Ok(())
}
