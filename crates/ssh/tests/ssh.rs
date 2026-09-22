use trex_ssh::connection::SshConnection;
use trex_ssh::reconnect::ReconnectLadder;
use trex_ssh::{PortForward, SshConnectionState, SshTarget};

fn target() -> SshTarget {
    SshTarget {
        id: "t1".to_string(),
        host: "example.test".to_string(),
        port: 22,
        username: "deploy".to_string(),
        identity_file: Some("/keys/id".to_string()),
        config_host: None,
        proxy_command: None,
        jump_host: None,
        port_forwards: vec![PortForward {
            local_port: 3000,
            remote_host: "localhost".to_string(),
            remote_port: 8080,
        }],
    }
}

#[tokio::test]
async fn connection_transitions_from_disconnected_to_connected() {
    let mut conn = SshConnection::new(target());
    assert_eq!(*conn.state(), SshConnectionState::Disconnected);
    assert!(!conn.is_connected());
    assert_eq!(conn.info().reconnect_attempts, 0);

    conn.connect().await.unwrap();
    assert!(conn.is_connected());
    assert_eq!(conn.info().state, SshConnectionState::Connected);
    assert!(conn.info().connected_at.is_some());
    assert_eq!(conn.target().host, "example.test");
}

#[test]
fn reconnect_ladder_backs_off_and_eventually_gives_up() {
    let mut ladder = ReconnectLadder::new();
    assert_eq!(ladder.next_delay(), Some(std::time::Duration::from_millis(1000)));
    assert!(!ladder.should_give_up());

    for _ in 0..9 {
        ladder.mark_attempt_failed();
    }
    assert!(ladder.should_give_up());
    assert_eq!(ladder.next_delay(), None);

    let mut fresh = ReconnectLadder::new();
    fresh.mark_attempt_failed();
    fresh.mark_attempt_failed();
    assert_eq!(fresh.next_delay(), Some(std::time::Duration::from_millis(5000)));
}

#[test]
fn reconnect_ladder_resets_after_stable_period() {
    let mut ladder = ReconnectLadder::new();
    let now = chrono::Utc::now();
    ladder.mark_connected(now);
    ladder.mark_attempt_failed();
    assert_eq!(ladder.next_delay(), Some(std::time::Duration::from_millis(2000)));

    ladder.reset_if_stable(now + chrono::Duration::seconds(30));
    assert_eq!(ladder.next_delay(), Some(std::time::Duration::from_millis(2000)));

    ladder.reset_if_stable(now + chrono::Duration::seconds(120));
    assert_eq!(ladder.next_delay(), Some(std::time::Duration::from_millis(1000)));
}

#[test]
fn ssh_target_serde_round_trip() {
    let json = serde_json::to_string(&target()).unwrap();
    let back: SshTarget = serde_json::from_str(&json).unwrap();
    assert_eq!(back.id, "t1");
    assert_eq!(back.port_forwards[0].local_port, 3000);
    assert!(back.jump_host.is_none());
}

#[test]
fn nested_jump_host_serde() {
    let mut t = target();
    t.jump_host = Some(Box::new(target()));
    let json = serde_json::to_string(&t).unwrap();
    let back: SshTarget = serde_json::from_str(&json).unwrap();
    assert_eq!(back.jump_host.unwrap().host, "example.test");
}