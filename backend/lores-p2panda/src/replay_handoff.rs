//! Stitches a bounded named-cursor replay together with an already-running
//! live subscription, so a caller sees replayed history followed by a
//! seamless, gap-free, duplicate-free live stream.
//!
//! Not yet wired into `lores-p2panda-server`'s `Subscribe` RPC — the
//! proto/request shape needed to ask for a replay doesn't exist yet.
#![allow(dead_code)]

use p2panda::operation::LogId;
use p2panda_core::logs::LogHeights;
use p2panda_core::{Cursor, VerifyingKey};
use tokio::sync::{broadcast, mpsc};
use tracing::warn;

use crate::panda_node::{IncomingOperation, PandaNode, SubscriptionError, SubscriptionEvent};
use crate::region::{RegionAppTopic, RegionTopic};

/// Replays `region_app_topic` under `cursor_name`, then hands off to `live`
/// for ongoing delivery once the replay ends, filtering out anything `live`
/// re-delivers that the replay already covered.
///
/// `live` must already be subscribed (i.e. obtained before this call) so no
/// operations published between subscribing and replaying are missed.
pub async fn replay_then_live(
    node: &PandaNode,
    region_app_topic: &RegionAppTopic,
    cursor_name: String,
    mut live: broadcast::Receiver<IncomingOperation>,
) -> Result<mpsc::Receiver<SubscriptionEvent>, SubscriptionError> {
    // p2panda's replay stream silently emits nothing at all (neither
    // ReplayStarted nor ReplayEnded) for a topic with no history, rather than
    // an empty replay — so we check upfront and skip it entirely rather than
    // waiting forever for an event that will never come.
    let has_history = node.has_operations_for_topic(region_app_topic.p2panda_topic()).await.map_err(|e| {
        tracing::error!("failed to check operation history for topic: {e}");
        SubscriptionError::ServerError
    })?;

    let mut replay_rx = if has_history {
        let (replay_tx, replay_rx) = mpsc::channel::<SubscriptionEvent>(32);
        node.replay_region_topic_as(region_app_topic, cursor_name, replay_tx).await?;
        replay_rx
    } else {
        mpsc::channel::<SubscriptionEvent>(1).1
    };

    let (out_tx, out_rx) = mpsc::channel::<SubscriptionEvent>(32);

    tokio::spawn(async move {
        let mut state = Cursor::<VerifyingKey, LogId>::new("replay-handoff-dedup", LogHeights::default());
        let mut backlog: Vec<IncomingOperation> = Vec::new();
        let mut replaying = has_history;

        if !has_history {
            // Synthesize the lifecycle events callers would otherwise expect
            // from a (skipped) replay, then fall straight through to live.
            if out_tx.send(SubscriptionEvent::ReplayStarted { total_operations: 0 }).await.is_err() {
                return;
            }
            if out_tx.send(SubscriptionEvent::ReplayEnded).await.is_err() {
                return;
            }
        }

        loop {
            tokio::select! {
                replay_event = replay_rx.recv(), if replaying => {
                    let Some(event) = replay_event else {
                        // Defensive fallback: treat unexpected channel closure
                        // the same as ReplayEnded so hand-off to live still happens.
                        replaying = false;
                        if forward_backlog(&mut backlog, &state, &out_tx).await.is_err() {
                            return;
                        }
                        continue;
                    };
                    match event {
                        SubscriptionEvent::Operation(op) => {
                            if let (Some(log_id), Some(seq_num)) = (op.log_id, op.seq_num) {
                                state.advance(op.author, log_id, seq_num);
                            }
                            if out_tx.send(SubscriptionEvent::Operation(op)).await.is_err() {
                                return;
                            }
                        }
                        SubscriptionEvent::ReplayStarted { .. } => {
                            if out_tx.send(event).await.is_err() {
                                return;
                            }
                        }
                        SubscriptionEvent::ReplayEnded => {
                            replaying = false;
                            if out_tx.send(event).await.is_err() {
                                return;
                            }
                            if forward_backlog(&mut backlog, &state, &out_tx).await.is_err() {
                                return;
                            }
                        }
                    }
                }
                live_event = live.recv() => {
                    let op = match live_event {
                        Ok(op) => op,
                        Err(broadcast::error::RecvError::Lagged(skipped)) => {
                            warn!("replay hand-off lagged behind live subscription, skipped {skipped} operations");
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    };

                    if replaying {
                        backlog.push(op);
                        continue;
                    }
                    if is_covered(&state, &op) {
                        continue;
                    }
                    if out_tx.send(SubscriptionEvent::Operation(Box::new(op))).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    Ok(out_rx)
}

/// Forwards every backlogged live operation not already covered by `state`.
async fn forward_backlog(
    backlog: &mut Vec<IncomingOperation>,
    state: &Cursor<VerifyingKey, LogId>,
    out_tx: &mpsc::Sender<SubscriptionEvent>,
) -> Result<(), ()> {
    for op in backlog.drain(..) {
        if is_covered(state, &op) {
            continue;
        }
        out_tx.send(SubscriptionEvent::Operation(Box::new(op))).await.map_err(|_| ())?;
    }
    Ok(())
}

/// Whether `op` was already delivered via the replay side, per `state`.
/// Ephemeral operations (no log position) are never covered by replay.
fn is_covered(state: &Cursor<VerifyingKey, LogId>, op: &IncomingOperation) -> bool {
    match (op.log_id, op.seq_num) {
        (Some(log_id), Some(seq_num)) => state.log_height(&op.author, &log_id) >= Some(&seq_num),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use p2panda_core::Hash;
    use tokio::sync::mpsc as tokio_mpsc;

    use super::*;
    use crate::panda_node::RequiredNodeParams;
    use crate::region::RegionId;

    async fn spawn_test_node() -> PandaNode {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("node.sqlite");
        std::mem::forget(dir); // keep the tempdir alive for the node's lifetime

        let params = RequiredNodeParams {
            credentials: p2panda::Credentials::generate(),
            network_id: Hash::from_bytes([0u8; 32]),
            bootstrap_node_ids: vec![],
            relay_url: None,
        };
        PandaNode::new(&params, db_path.to_str().unwrap()).await.unwrap()
    }

    /// Subscribes live to `topic` and bridges the resulting operations onto a
    /// broadcast channel, mirroring how `PandaService::ensure_broadcast_subscription`
    /// wires up its shared live subscription in production.
    async fn subscribe_live(node: &PandaNode, topic: &RegionAppTopic) -> broadcast::Receiver<IncomingOperation> {
        subscribe_live_with_capacity(node, topic, 128).await
    }

    /// Like `subscribe_live`, but with a caller-chosen broadcast capacity, so
    /// tests can force a `Lagged` receiver by publishing past it.
    async fn subscribe_live_with_capacity(
        node: &PandaNode,
        topic: &RegionAppTopic,
        capacity: usize,
    ) -> broadcast::Receiver<IncomingOperation> {
        let (incoming_tx, mut incoming_rx) = tokio_mpsc::channel::<IncomingOperation>(32);
        node.subscribe_to_region_topic(topic, incoming_tx).await.unwrap();

        let (broadcast_tx, broadcast_rx) = broadcast::channel::<IncomingOperation>(capacity);
        tokio::spawn(async move {
            while let Some(op) = incoming_rx.recv().await {
                let _ = broadcast_tx.send(op);
            }
        });

        broadcast_rx
    }

    async fn next_event(rx: &mut mpsc::Receiver<SubscriptionEvent>) -> SubscriptionEvent {
        tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("timed out waiting for hand-off event")
            .expect("hand-off channel closed unexpectedly")
    }

    async fn next_operation_bytes(rx: &mut mpsc::Receiver<SubscriptionEvent>) -> Vec<u8> {
        match next_event(rx).await {
            SubscriptionEvent::Operation(op) => op.bytes,
            SubscriptionEvent::ReplayStarted { .. } => panic!("expected an operation event, got ReplayStarted"),
            SubscriptionEvent::ReplayEnded => panic!("expected an operation event, got ReplayEnded"),
        }
    }

    /// Asserts no event arrives within a short window, without waiting out
    /// `next_event`'s full timeout.
    async fn expect_no_event(rx: &mut mpsc::Receiver<SubscriptionEvent>) {
        assert!(
            tokio::time::timeout(Duration::from_millis(200), rx.recv()).await.is_err(),
            "expected no further hand-off events"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn replays_history_then_stays_idle_with_no_live_operations() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        let live = subscribe_live(&node, &topic).await;
        node.publish_to_region_topic(&topic, b"before-1".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"before-2".to_vec()).await.unwrap();

        let mut handoff = replay_then_live(&node, &topic, "test-cursor".to_string(), live).await.unwrap();

        assert!(matches!(
            next_event(&mut handoff).await,
            SubscriptionEvent::ReplayStarted { total_operations: 2 }
        ));
        assert_eq!(next_operation_bytes(&mut handoff).await, b"before-1");
        assert_eq!(next_operation_bytes(&mut handoff).await, b"before-2");
        assert!(matches!(next_event(&mut handoff).await, SubscriptionEvent::ReplayEnded));

        expect_no_event(&mut handoff).await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn empty_history_then_live() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        let live = subscribe_live(&node, &topic).await;
        let mut handoff = replay_then_live(&node, &topic, "test-cursor".to_string(), live).await.unwrap();

        assert!(matches!(
            next_event(&mut handoff).await,
            SubscriptionEvent::ReplayStarted { total_operations: 0 }
        ));
        assert!(matches!(next_event(&mut handoff).await, SubscriptionEvent::ReplayEnded));

        node.publish_to_region_topic(&topic, b"after-1".to_vec()).await.unwrap();
        assert_eq!(next_operation_bytes(&mut handoff).await, b"after-1");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn replays_history_then_hands_off_to_live_without_duplicates() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        let live = subscribe_live(&node, &topic).await;
        node.publish_to_region_topic(&topic, b"before-1".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"before-2".to_vec()).await.unwrap();

        let mut handoff = replay_then_live(&node, &topic, "test-cursor".to_string(), live).await.unwrap();

        assert!(matches!(
            next_event(&mut handoff).await,
            SubscriptionEvent::ReplayStarted { total_operations: 2 }
        ));
        assert_eq!(next_operation_bytes(&mut handoff).await, b"before-1");
        assert_eq!(next_operation_bytes(&mut handoff).await, b"before-2");
        assert!(matches!(next_event(&mut handoff).await, SubscriptionEvent::ReplayEnded));

        node.publish_to_region_topic(&topic, b"after-1".to_vec()).await.unwrap();
        assert_eq!(next_operation_bytes(&mut handoff).await, b"after-1");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn overlap_between_live_subscribe_and_replay_snapshot_is_not_duplicated() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        let live = subscribe_live(&node, &topic).await;
        node.publish_to_region_topic(&topic, b"before-1".to_vec()).await.unwrap();

        // Further operations land before replay's snapshot point is taken —
        // these should show up exactly once, via replay, and be skipped when
        // they're re-delivered live.
        node.publish_to_region_topic(&topic, b"overlap-1".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"overlap-2".to_vec()).await.unwrap();

        let mut handoff = replay_then_live(&node, &topic, "test-cursor".to_string(), live).await.unwrap();

        let mut replayed = Vec::new();
        loop {
            match next_event(&mut handoff).await {
                SubscriptionEvent::ReplayStarted { .. } => continue,
                SubscriptionEvent::Operation(op) => replayed.push(op.bytes),
                SubscriptionEvent::ReplayEnded => break,
            }
        }
        assert_eq!(replayed, vec![b"before-1".to_vec(), b"overlap-1".to_vec(), b"overlap-2".to_vec()]);

        node.publish_to_region_topic(&topic, b"after-1".to_vec()).await.unwrap();
        assert_eq!(next_operation_bytes(&mut handoff).await, b"after-1");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn tolerates_lagged_live_receiver_during_overlap() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        // Small capacity so the live receiver reliably lags once we publish
        // past it without ever draining it.
        let live = subscribe_live_with_capacity(&node, &topic, 2).await;

        node.publish_to_region_topic(&topic, b"overlap-1".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"overlap-2".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"overlap-3".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"overlap-4".to_vec()).await.unwrap();

        let mut handoff = replay_then_live(&node, &topic, "test-cursor".to_string(), live).await.unwrap();

        // Replay reads straight from persisted history, so it's unaffected by
        // the live receiver having lagged and dropped some of these.
        assert!(matches!(
            next_event(&mut handoff).await,
            SubscriptionEvent::ReplayStarted { total_operations: 4 }
        ));
        for expected in ["overlap-1", "overlap-2", "overlap-3", "overlap-4"] {
            assert_eq!(next_operation_bytes(&mut handoff).await, expected.as_bytes());
        }
        assert!(matches!(next_event(&mut handoff).await, SubscriptionEvent::ReplayEnded));

        // The lag was tolerated rather than treated as fatal: the hand-off
        // task is still alive and forwarding new live operations.
        node.publish_to_region_topic(&topic, b"after-1".to_vec()).await.unwrap();
        assert_eq!(next_operation_bytes(&mut handoff).await, b"after-1");
    }
}
