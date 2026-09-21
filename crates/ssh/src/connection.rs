use crate::types::{SshConnectionInfo, SshConnectionState, SshTarget};
use anyhow::Result;
use std::time::Duration;
use tracing::{info, warn};

pub struct SshConnection {
    target: SshTarget,
    state: SshConnectionState,
    info: SshConnectionInfo,
}

impl SshConnection {
    pub fn new(target: SshTarget) -> Self {
        let info = SshConnectionInfo {
            state: SshConnectionState::Disconnected,
            target_id: target.id.clone(),
            connected_at: None,
            reconnect_attempts: 0,
            last_error: None,
        };
        SshConnection {
            target,
            state: SshConnectionState::Disconnected,
            info,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.state = SshConnectionState::Connecting;
        self.info.state = SshConnectionState::Connecting;

        info!(target = %self.target.host, "Connecting via SSH");

        // TODO: Implement actual SSH connection via tokio-ssh
        // For now, stub the connection flow
        self.state = SshConnectionState::Connected;
        self.info.state = SshConnectionState::Connected;
        self.info.connected_at = Some(chrono::Utc::now());
        self.info.reconnect_attempts = 0;

        Ok(())
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        let backoff = self.reconnect_backoff();
        warn!(
            target = %self.target.host,
            attempt = self.info.reconnect_attempts,
            delay_ms = backoff.as_millis() as u64,
            "Reconnecting"
        );

        tokio::time::sleep(backoff).await;
        self.info.reconnect_attempts += 1;
        self.connect().await
    }

    fn reconnect_backoff(&self) -> Duration {
        let attempts = self.info.reconnect_attempts;
        let delays = [1000, 2000, 5000, 5000, 10000, 10000, 10000, 30000, 30000];
        let idx = attempts.min(delays.len() - 1);
        Duration::from_millis(delays[idx])
    }

    pub fn is_connected(&self) -> bool {
        self.state == SshConnectionState::Connected
    }

    pub fn state(&self) -> &SshConnectionState {
        &self.state
    }

    pub fn info(&self) -> &SshConnectionInfo {
        &self.info
    }

    pub fn target(&self) -> &SshTarget {
        &self.target
    }
}
