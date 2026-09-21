use crate::connection::SshConnection;
use crate::types::{RelayState, SshTarget};
use anyhow::Result;
use tracing::info;

pub struct SshRelaySession {
    target: SshTarget,
    connection: SshConnection,
    state: RelayState,
}

impl SshRelaySession {
    pub fn new(target: SshTarget) -> Self {
        let connection = SshConnection::new(target.clone());
        SshRelaySession {
            target,
            connection,
            state: RelayState::Idle,
        }
    }

    pub async fn establish(&mut self) -> Result<()> {
        self.state = RelayState::Deploying;
        info!(target = %self.target.host, "Deploying relay");

        self.connection.connect().await?;

        // TODO: Upload relay binary, launch, create mux
        self.state = RelayState::Ready;
        info!(target = %self.target.host, "Relay ready");
        Ok(())
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        self.state = RelayState::Reconnecting;
        self.connection.reconnect().await?;

        // TODO: Re-deploy relay, re-register providers
        self.state = RelayState::Ready;
        Ok(())
    }

    pub fn state(&self) -> &RelayState {
        &self.state
    }

    pub fn is_ready(&self) -> bool {
        self.state == RelayState::Ready
    }
}
