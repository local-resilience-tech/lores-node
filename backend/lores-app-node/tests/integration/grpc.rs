use std::time::Duration;

use lores_app_node::AppNode;

use crate::common::{TestOp, start_dev_server};

/// An operation published by one node is delivered to another node subscribed
/// to the same app over gRPC, exercised against the in-memory dev server.
#[tokio::test]
async fn publishes_and_receives_operation_over_grpc() {
    let endpoint = start_dev_server().await;
    let app_id = "grpc-test-app";

    let publisher = AppNode::<TestOp>::grpc(endpoint.clone(), app_id, "publisher").unwrap();
    let subscriber = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber").unwrap();

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
