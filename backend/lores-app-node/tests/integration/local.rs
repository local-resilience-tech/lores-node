use lores_app_node::{AppNode, NodeEvent};
use lores_p2panda_client::SubscriptionFrom;

use crate::common::{TestOp, drain_events, memory_pool};

/// A node with no persisted history, subscribing with `SubscriptionFrom::Start`,
/// still receives the replay lifecycle events around the (empty) replay.
#[tokio::test]
async fn local_node_with_no_history_emits_replay_lifecycle_events() {
    let node = AppNode::<TestOp>::local(memory_pool().await, "local-test-app", "instance", SubscriptionFrom::Start)
        .await
        .unwrap();

    let mut node_events = node.subscribe_node_events();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received = drain_events(&mut node_events).await;

    assert!(matches!(received.len(), 2));
    assert!(matches!(received[0], NodeEvent::ReplayStarted { total_operations: 0 }));
    assert!(matches!(received[1], NodeEvent::ReplayEnded));
}
