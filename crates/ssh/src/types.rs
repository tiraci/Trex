use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshTarget {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub identity_file: Option<String>,
    pub config_host: Option<String>,
    pub proxy_command: Option<String>,
    pub jump_host: Option<Box<SshTarget>>,
    pub port_forwards: Vec<PortForward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForward {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SshConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    AuthFailed,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConnectionInfo {
    pub state: SshConnectionState,
    pub target_id: String,
    pub connected_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reconnect_attempts: usize,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelayState {
    Idle,
    Deploying,
    Ready,
    Reconnecting,
    Disposed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshRemotePtyLease {
    pub lease_id: String,
    pub target_id: String,
    pub pane_key: String,
    pub state: PtyLeaseState,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PtyLeaseState {
    Attached,
    Detached,
    Terminated,
    Expired,
}
