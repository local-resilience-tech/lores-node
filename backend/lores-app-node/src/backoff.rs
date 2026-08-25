use std::time::Duration;

use tokio::sync::watch;

use crate::node::NodeError;

/// Exponential backoff state that resets when the error variant changes.
pub(crate) struct Backoff {
    pub(crate) current: Duration,
    last_error: Option<std::mem::Discriminant<NodeError>>,
}

impl Backoff {
    const MIN: Duration = Duration::from_secs(1);
    const MAX: Duration = Duration::from_secs(60);

    pub(crate) fn new() -> Self {
        Self {
            current: Self::MIN,
            last_error: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.current = Self::MIN;
        self.last_error = None;
    }

    /// Set `error` on `error_tx`, reset the duration if the variant changed,
    /// sleep for the current duration, then double it (up to `MAX`).
    pub(crate) async fn set_error_and_advance(
        &mut self,
        error_tx: &watch::Sender<Option<NodeError>>,
        error: NodeError,
    ) {
        let d = std::mem::discriminant(&error);
        if self.last_error != Some(d) {
            self.current = Self::MIN;
            self.last_error = Some(d);
        }
        error_tx.send_replace(Some(error));
        tokio::time::sleep(self.current).await;
        self.current = (self.current * 2).min(Self::MAX);
    }
}
