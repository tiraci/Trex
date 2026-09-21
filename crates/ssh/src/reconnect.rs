use crate::types::SshConnectionState;
use std::time::Duration;

pub struct ReconnectLadder {
    delay_index: usize,
    consecutive_failures: usize,
    stable_since: Option<chrono::DateTime<chrono::Utc>>,
}

const BACKOFF_MS: [u64; 9] = [1000, 2000, 5000, 5000, 10000, 10000, 10000, 30000, 30000];
const STABLE_THRESHOLD_SECS: i64 = 60;

impl ReconnectLadder {
    pub fn new() -> Self {
        ReconnectLadder {
            delay_index: 0,
            consecutive_failures: 0,
            stable_since: None,
        }
    }

    pub fn mark_connected(&mut self, now: chrono::DateTime<chrono::Utc>) {
        self.consecutive_failures = 0;
        self.stable_since = Some(now);
    }

    pub fn mark_attempt_failed(&mut self) {
        self.consecutive_failures += 1;
        self.delay_index = (self.delay_index + 1).min(BACKOFF_MS.len() - 1);
    }

    pub fn next_delay(&self) -> Option<Duration> {
        if self.consecutive_failures >= BACKOFF_MS.len() {
            return None;
        }
        Some(Duration::from_millis(BACKOFF_MS[self.delay_index]))
    }

    pub fn should_give_up(&self) -> bool {
        self.consecutive_failures >= BACKOFF_MS.len()
    }

    pub fn reset_if_stable(&mut self, now: chrono::DateTime<chrono::Utc>) {
        if let Some(stable_since) = self.stable_since {
            if (now - stable_since).num_seconds() >= STABLE_THRESHOLD_SECS {
                self.delay_index = 0;
            }
        }
    }
}
