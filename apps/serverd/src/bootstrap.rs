use whale_config::ServerConfig;
use whale_tcp::{TcpLimits, TcpServer};

pub async fn run() -> anyhow::Result<()> {
    let config = ServerConfig {
        listen_address: std::env::var("WHALE_LISTEN_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned()),
        ..ServerConfig::default()
    };

    config.validate()?;

    let limits = TcpLimits {
        max_connections: config.max_connections,
        connection_timeout: config.connection_timeout,
        shutdown_timeout: config.shutdown_timeout,
    };

    let server = TcpServer::bind(&config.listen_address, limits).await?;

    tracing::info!(
        address = %config.listen_address,
        max_connections = config.max_connections,
        "Whale server listening"
    );

    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let mut server_task = tokio::spawn(async move {
        server
            .run(shutdown_rx, |connection| async move {
                tracing::info!(
                    peer = %connection.peer_addr(),
                    "TCP connection accepted"
                );

                Ok(())
            })
            .await
    });

    tokio::select! {
        result = &mut server_task => {
            result??;
        }

        signal = crate::shutdown::wait_for_signal() => {
            signal?;

            tracing::info!("Shutdown requested");

            let _ = shutdown_tx.send(true);

            server_task.await??;
        }
    }

    tracing::info!("Whale server stopped");

    Ok(())
}
