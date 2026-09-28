use lores_app_node::{AppNode, NodeEvent};
use lores_p2panda_client::SubscriptionFrom;
use pretty_assertions::assert_eq;

use crate::common::{TestOp, drain_node_events, drain_operations, memory_pool};

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

    let received = drain_node_events(&mut node_events).await;

    assert_eq!(received.len(), 2, "received: {received:?}");
    assert_eq!(received[0], NodeEvent::ReplayStarted { total_operations: 0 });
    assert_eq!(received[1], NodeEvent::ReplayEnded);
}

/// A node with one persisted operation, subscribing with `SubscriptionFrom::Start`,
/// replays that operation between the lifecycle events.
#[tokio::test]
async fn local_node_with_one_operation_replays_it_from_start() {
    let node = AppNode::<TestOp>::local(memory_pool().await, "local-test-app", "instance", SubscriptionFrom::Start)
        .await
        .unwrap();

    let op = TestOp { msg: "persisted".into() };
    node.publish(&op).await.unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received = drain_node_events(&mut node_events).await;

    assert_eq!(received.len(), 2, "received: {received:?}");
    assert_eq!(received[0], NodeEvent::ReplayStarted { total_operations: 1 });
    assert_eq!(received[1], NodeEvent::ReplayEnded);

    let replayed = drain_operations(&mut operations).await;
    assert_eq!(replayed.len(), 1);
    assert_eq!(replayed[0].op, op);
}
