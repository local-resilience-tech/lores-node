use lores_app_node::{AppNode, NodeEvent};
use lores_p2panda_client::SubscriptionFrom;
use pretty_assertions::assert_eq;
use std::time::Duration;

use crate::common::{
    TestOp, drain_node_events, drain_operations, insert_operation_into_db, memory_pool, memory_pool_with_schema, start_dev_server,
};

// SubscriptionFrom::Frontier tests

#[tokio::test]
async fn outbox_from_frontier_with_no_operations_emits_nothing_but_connection() {
    let endpoint = start_dev_server().await;
    let app_id = "outbox-test-app";

    let node = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber", SubscriptionFrom::Frontier).unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;
    assert_eq!(received_ne.len(), 1);
    assert!(matches!(received_ne[0], NodeEvent::ServerConnected { .. }));

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 0);
}

#[tokio::test]
async fn outbox_from_frontier_with_remote_operation_emits_it() {
    let endpoint = start_dev_server().await;
    let app_id = "outbox-test-app";

    let publisher = AppNode::<TestOp>::grpc_with_local(
        memory_pool().await,
        endpoint.clone(),
        app_id,
        "publisher",
        SubscriptionFrom::Frontier,
    )
    .await
    .unwrap();

    let node = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber", SubscriptionFrom::Frontier).unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    tokio::time::sleep(Duration::from_millis(300)).await;

    let op = TestOp { msg: "via outbox".into() };
    publisher.publish(&op).await.unwrap();

    let received_ne = drain_node_events(&mut node_events).await;
    assert_eq!(received_ne.len(), 1);
    assert!(matches!(received_ne[0], NodeEvent::ServerConnected { .. }));

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 1);
    assert_eq!(received_ops[0].op, op);
}

#[tokio::test]
async fn outbox_from_frontier_with_local_operation_emits_nothing() {
    let endpoint = start_dev_server().await;
    let app_id = "outbox-test-app";

    let pool = memory_pool_with_schema().await;

    let op = TestOp {
        msg: "pre-seeded local".into(),
    };
    let payload = serde_json::to_vec(&op).unwrap();
    insert_operation_into_db(&pool, payload).await;

    let node = AppNode::<TestOp>::grpc_with_local(pool, endpoint.clone(), app_id, "subscriber", SubscriptionFrom::Frontier)
        .await
        .unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;
    assert_eq!(received_ne.len(), 1);
    assert!(matches!(received_ne[0], NodeEvent::ServerConnected { .. }));

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 0);
}

// SubscriptionFrom::Start tests

#[tokio::test]
async fn outbox_from_start_with_local_operation_emits_it() {
    let endpoint = start_dev_server().await;
    let app_id = "outbox-test-app";

    let pool = memory_pool_with_schema().await;

    let op = TestOp {
        msg: "pre-seeded local".into(),
    };
    let payload = serde_json::to_vec(&op).unwrap();
    insert_operation_into_db(&pool, payload).await;

    let node = AppNode::<TestOp>::grpc_with_local(pool, endpoint.clone(), app_id, "subscriber", SubscriptionFrom::Start)
        .await
        .unwrap();

    let mut node_events = node.subscribe_node_events();
    let mut operations = node.subscribe();

    let driver = node.clone();
    tokio::spawn(async move { driver.run().await });

    let received_ne = drain_node_events(&mut node_events).await;
    assert_eq!(received_ne.len(), 3);
    assert!(matches!(received_ne[0], NodeEvent::ServerConnected { .. }));
    assert_eq!(received_ne[1], NodeEvent::ReplayStarted { total_operations: 0 });
    assert_eq!(received_ne[2], NodeEvent::ReplayEnded);

    let received_ops = drain_operations(&mut operations).await;
    assert_eq!(received_ops.len(), 1);
    assert_eq!(received_ops[0].op, op);
}
