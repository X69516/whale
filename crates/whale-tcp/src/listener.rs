use std::{future::Future, io, sync::Arc};

use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::{Semaphore, watch},
    task::JoinSet,
    time::timeout,
};

use crate::{connection::TcpConnection, limits::TcpLimits};

pub struct TcpServer {
    listener: TcpListener,
    limits: TcpLimits,
}

impl TcpServer {
    pub async fn bind(address: impl ToSocketAddrs, limits: TcpLimits) -> io::Result<Self> {
        limits.validate()?;

        let listener = TcpListener::bind(address).await?;

        Ok(Self { listener, limits })
    }

    pub async fn run<F, Fut>(
        self,
        mut shutdown: watch::Receiver<bool>,
        handler: F,
    ) -> io::Result<()>
    where
        F: Fn(TcpConnection) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = io::Result<()>> + Send + 'static,
    {
        let semaphore = Arc::new(Semaphore::new(self.limits.max_connections));

        let handler = Arc::new(handler);
        let mut tasks = JoinSet::new();

        loop {
            if *shutdown.borrow() {
                break;
            }

            tokio::select! {
                biased;

                result = shutdown.changed() => {
                    if result.is_err() || *shutdown.borrow() {
                        break;
                    }
                }

                accepted = self.listener.accept() => {
                    let (stream, peer) = accepted?;

                    let permit = match semaphore
                        .clone()
                        .try_acquire_owned()
                    {
                        Ok(permit) => permit,
                        Err(_) => {
                            // Connection limit reached.
                            // Dropping stream closes the connection.
                            drop(stream);
                            continue;
                        }
                    };

                    let connection = TcpConnection::new(
                        stream,
                        peer,
                    );

                    let handler = Arc::clone(&handler);
                    let connection_timeout =
                        self.limits.connection_timeout;

                    tasks.spawn(async move {
                        let _permit = permit;

                        let _ = timeout(
                            connection_timeout,
                            handler(connection),
                        ).await;
                    });
                }

                Some(_) = tasks.join_next(), if !tasks.is_empty() => {}
            }
        }

        // Allow active connections to finish.
        let shutdown_timeout = self.limits.shutdown_timeout;

        let _ = timeout(shutdown_timeout, async {
            while tasks.join_next().await.is_some() {}
        })
        .await;

        // Abort remaining tasks after timeout.
        tasks.abort_all();

        while tasks.join_next().await.is_some() {}

        Ok(())
    }
}
