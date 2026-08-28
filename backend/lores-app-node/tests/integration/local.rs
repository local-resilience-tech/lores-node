use std::time::Duration;

use lores_app_node::AppNode;

use crate::common::{memory_pool, TestOp};

/// A local-only node broadcasts published operations to loopback subscribers
/// and re-emits persisted operations on replay.
#[tokio::test]
async fn local_node_broadcasts_and_replays() {
    let node = AppNode::<TestOp>::local(memory_pool().await, "local-test-app", "instance")
        .await
        .unwrap();

    let mut events = node.subscribe();

    let op = TestOp { msg: "persisted".into() };
    node.publish(&op).await.unwrap();

    let received = tokio::time::timeout(Duration::from_secs(5), events.recv())
        .await
        .expect("timed out waiting for loopback")
        .expect("event channel closed");
    assert_eq!(received.op, op);

    // A subscriber attached after publishing receives the operation only via replay.
    let mut replayed = node.subscribe();
    node.replay().await.unwrap();

    let r = tokio::time::timeout(Duration::from_secs(5), replayed.recv())
        .await
        .expect("timed out waiting for replay")
        .expect("event channel closed");
    assert_eq!(r.op, op);
}
