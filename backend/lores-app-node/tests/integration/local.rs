use lores_app_node::{AppNode, NodeEvent};
use lores_p2panda_client::SubscriptionFrom;
use pretty_assertions::assert_eq;

use crate::common::{TestOp, drain_node_events, drain_operations, memory_pool};

// SubscriptionFrom::Frontier tests

#[tokio::test]
async fn local_node_from_frontier_with_one_operation_does_not_emit_it() {
    let node = AppNode::<TestOp>::local(memory_pool().await, "local-test-app", "instance", SubscriptionFrom::Frontier)
        .await
        .unwrap();

    let op = TestOp { msg: "persisted".into() };
    node.publish(&op).await.unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;
    assert_eq!(received_ne.len(), 0);

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 0);
}

// SubscriptionFrom::Start tests

/// A node with no persisted history, subscribing with `SubscriptionFrom::Start`,
/// still receives the replay lifecycle events around the (empty) replay.
#[tokio::test]
async fn local_node_from_startwith_no_history_emits_replay_lifecycle_events() {
    let node = AppNode::<TestOp>::local(memory_pool().await, "local-test-app", "instance", SubscriptionFrom::Start)
        .await
        .unwrap();

    let mut node_events = node.subscribe_node_events();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;

    assert_eq!(received_ne.len(), 2, "received: {received_ne:?}");
    assert_eq!(received_ne[0], NodeEvent::ReplayStarted { total_operations: 0 });
    assert_eq!(received_ne[1], NodeEvent::ReplayEnded);
}

/// A node with one persisted operation, subscribing with `SubscriptionFrom::Start`,
/// replays that operation between the lifecycle events.
#[tokio::test]
async fn local_node_from_start_with_one_operation_replays_it() {
    let node = AppNode::<TestOp>::local(memory_pool().await, "local-test-app", "instance", SubscriptionFrom::Start)
        .await
        .unwrap();

    let op = TestOp { msg: "persisted".into() };
    node.publish(&op).await.unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;

    assert_eq!(received_ne.len(), 2);
    assert_eq!(received_ne[0], NodeEvent::ReplayStarted { total_operations: 1 });
    assert_eq!(received_ne[1], NodeEvent::ReplayEnded);

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 1);
    assert_eq!(received_ops[0].op, op);
}
