use tokio::net::UnixListener;
use anyhow::Result;
use log::{info, error};
use std::path::Path;
use tokio::io::AsyncReadExt;

pub struct DaemonIpc {
    socket_path: String,
}

impl DaemonIpc {
    pub fn new(socket_path: &str) -> Self {
        Self {
            socket_path: socket_path.to_string(),
        }
    }

    pub async fn start<F, Fut>(&self, mut on_trigger: F) -> Result<()>
    where
        F: FnMut(String) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        if Path::new(&self.socket_path).exists() {
            std::fs::remove_file(&self.socket_path)?;
        }

        let listener = UnixListener::bind(&self.socket_path)?;
        info!("Daemon listening for IPC on {}", self.socket_path);

        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => {
                    let mut buf = [0; 1024];
                    if let Ok(n) = stream.read(&mut buf).await {
                        let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        if msg.starts_with("trigger-scan") || msg == "show-help" {
                            info!("Received IPC msg: {}", msg);
                            on_trigger(msg).await;
                        }
                    }
                }
                Err(e) => error!("IPC error: {}", e),
            }
        }
    }
}
