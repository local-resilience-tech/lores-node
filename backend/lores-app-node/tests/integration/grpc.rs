use lores_app_node::{AppNode, NodeEvent};
use lores_p2panda_client::SubscriptionFrom;
use pretty_assertions::assert_eq;
use std::time::Duration;

use crate::common::{TestOp, drain_node_events, drain_operations, start_dev_server, start_dev_server_with_service};

// SubscriptionFrom::Frontier tests

/// An operation published by one node is delivered to another node subscribed
/// to the same app over gRPC, exercised against the in-memory dev server.
#[tokio::test]
async fn publishes_and_receives_operation_over_grpc() {
    let endpoint = start_dev_server().await;
    let app_id = "grpc-test-app";

    let publisher = AppNode::<TestOp>::grpc(endpoint.clone(), app_id, "publisher", SubscriptionFrom::Frontier).unwrap();
    let subscriber = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber", SubscriptionFrom::Frontier).unwrap();

    let mut events = subscriber.subscribe();

    let driver = subscriber.clone();
    tokio::spawn(async move { driver.run().await });

    // The dev server only delivers operations that arrive after a subscription
    // is established, so wait for the subscriber to connect before publishing.
    tokio::time::sleep(Duration::from_millis(300)).await;

    let op = TestOp { msg: "hello".into() };
    publisher.publish(&op).await.unwrap();

    let received = tokio::time::timeout(Duration::from_secs(5), events.recv())
        .await
        .expect("timed out waiting for operation")
        .expect("event channel closed");

    assert_eq!(received.op, op);
    assert!(received.panda_operation_id.is_some());
}

/// The in-memory dev server has no persistence and ignores the subscription
/// cursor entirely (no replay lifecycle events, no history) — so this only
/// exercises that `SubscriptionFrom::Start` doesn't break normal pub/sub over
/// gRPC, not real replay-then-live semantics (that needs a real p2panda-backed
/// server).
#[tokio::test]
async fn publishes_and_receives_operation_with_start_cursor() {
    let endpoint = start_dev_server().await;
    let app_id = "grpc-test-app-start";

    let publisher = AppNode::<TestOp>::grpc(endpoint.clone(), app_id, "publisher", SubscriptionFrom::Start).unwrap();
    let subscriber = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber", SubscriptionFrom::Start).unwrap();

    let mut events = subscriber.subscribe();

    let driver = subscriber.clone();
    tokio::spawn(async move { driver.run().await });

    tokio::time::sleep(Duration::from_millis(300)).await;

    let op = TestOp {
        msg: "hello from start".into(),
    };
    publisher.publish(&op).await.unwrap();

    let received = tokio::time::timeout(Duration::from_secs(5), events.recv())
        .await
        .expect("timed out waiting for operation")
        .expect("event channel closed");

    assert_eq!(received.op, op);
    assert!(received.panda_operation_id.is_some());
}

// SubscriptionFrom::Start tests

/// A historical operation injected into the dev server is replayed to a
/// `SubscriptionFrom::Start` subscriber, surrounded by replay lifecycle events.
#[tokio::test]
async fn grpc_from_start_with_remote_operation_emits_it() {
    let (endpoint, service) = start_dev_server_with_service().await;
    let app_id = "grpc-start-replay-app";

    let op = TestOp { msg: "via replay".into() };
    service.inject_observed_operation(app_id, serde_json::to_vec(&op).unwrap()).await;

    let node = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber", SubscriptionFrom::Start).unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;
    assert_eq!(received_ne.len(), 3);
    assert!(matches!(received_ne[0], NodeEvent::ServerConnected { .. }));
    assert_eq!(received_ne[1], NodeEvent::ReplayStarted { total_operations: 1 });
    assert_eq!(received_ne[2], NodeEvent::ReplayEnded);

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 1);
    assert_eq!(received_ops[0].op, op);
}
