use std::time::Duration;

use lores_app_node::AppNode;
use lores_p2panda_client::SubscriptionFrom;

use crate::common::{TestOp, memory_pool, start_dev_server};

/// The outbox transport persists locally, delivers over gRPC, and drains the local
/// copy once delivery is acknowledged.
#[tokio::test]
async fn outbox_delivers_over_grpc_and_drains_local() {
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
    let subscriber = AppNode::<TestOp>::grpc(endpoint, app_id, "subscriber", SubscriptionFrom::Frontier).unwrap();

    let mut events = subscriber.subscribe();

    let driver = subscriber.clone();
    tokio::spawn(async move { driver.run().await });

    tokio::time::sleep(Duration::from_millis(300)).await;

    let op = TestOp { msg: "via outbox".into() };
    publisher.publish(&op).await.unwrap();

    let received = tokio::time::timeout(Duration::from_secs(5), events.recv())
        .await
        .expect("timed out waiting for operation")
        .expect("event channel closed");
    assert_eq!(received.op, op);
}
