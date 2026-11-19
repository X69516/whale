pub async fn wait_for_signal() -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut terminate = signal(SignalKind::terminate())?;

        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                result
            }

            _ = terminate.recv() => {
                Ok(())
            }
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await
    }
}
