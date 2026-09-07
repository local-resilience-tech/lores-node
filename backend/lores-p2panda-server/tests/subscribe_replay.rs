use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use lores_p2panda::{PandaNode, RegionAppTopic, RegionId, RequiredNodeParams};
use lores_p2panda_server::proto::panda_server::Panda;
use lores_p2panda_server::proto::{PublishRequest, SubscribeEvent, SubscribeRequest, subscribe_event::Event as SubscribeEventKind};
use lores_p2panda_server::{AppInstanceIds, IdempotencyConfig, NodeInfo, PandaService, ResolveNodeInfo, ResolveRegionId, ResolvedRegion};
use p2panda_core::{Hash, SigningKey};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tonic::Request;

fn resolve_region_id(region_id: RegionId) -> ResolveRegionId {
    Arc::new(move |_ids: AppInstanceIds| {
        let region_id = region_id.clone();
        Box::pin(async move {
            Ok(ResolvedRegion {
                region_id,
                slug: None,
                name: None,
            })
        }) as Pin<Box<dyn Future<Output = Result<ResolvedRegion, _>> + Send>>
    })
}

fn resolve_node_info() -> ResolveNodeInfo {
    Arc::new(|_ids: AppInstanceIds, _node_id: String| {
        Box::pin(async move {
            Ok(NodeInfo {
                node_id: "test".to_string(),
                name: None,
                domain_on_internet: None,
            })
        }) as Pin<Box<dyn Future<Output = Result<NodeInfo, _>> + Send>>
    })
}

async fn temp_db_pool(prefix: &str) -> (SqlitePool, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(format!("{prefix}.sqlite"));
    let options = SqliteConnectOptions::new().filename(&path).create_if_missing(true);
    let pool = SqlitePoolOptions::new().max_connections(2).connect_with(options).await.unwrap();
    (pool, dir)
}

async fn make_node(db_url: &str) -> Arc<PandaNode> {
    let private_key = SigningKey::generate();
    let network_id = Hash::from_bytes([0u8; 32]);
    let params = RequiredNodeParams {
        private_key,
        network_id,
        bootstrap_node_ids: vec![],
        relay_url: None,
    };
    Arc::new(PandaNode::new(&params, db_url).await.unwrap())
}

async fn make_service(node: Arc<PandaNode>, region_id: RegionId) -> PandaService {
    let (db, _dir) = temp_db_pool("idempotency").await;
    let wrapped_node = Arc::new(Mutex::new(Some(node)));
    PandaService::new(
        wrapped_node,
        db,
        Some(IdempotencyConfig {
            cleanup_frequency: Duration::from_secs(1),
            retention: Duration::from_secs(2),
        }),
        Arc::new(|_app_id, _instance_id| {}),
        resolve_region_id(region_id),
        resolve_node_info(),
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn subscribe_with_replay_yields_historical_then_live_operations() {
    let app_id = "test-app";
    let instance_id = "test-instance";
    let region_id = RegionId::generate();
    let _region_app_topic = RegionAppTopic::new(region_id.clone(), app_id);

    let (node_pool, _node_dir) = temp_db_pool("node").await;
    let node_db_url = format!("{}", node_pool.connect_options().get_filename().display());
    let node = make_node(&node_db_url).await;
    let service = make_service(node.clone(), region_id).await;

    // Publish two operations before subscribing with replay.
    let payload1 = b"first".to_vec();
    let payload2 = b"second".to_vec();
    service
        .publish(Request::new(PublishRequest {
            app_id: app_id.to_string(),
            instance_id: instance_id.to_string(),
            payload: payload1,
            idempotency_key: vec![],
        }))
        .await
        .unwrap();
    service
        .publish(Request::new(PublishRequest {
            app_id: app_id.to_string(),
            instance_id: instance_id.to_string(),
            payload: payload2,
            idempotency_key: vec![],
        }))
        .await
        .unwrap();

    // Subscribe with replay.
    let response = service
        .subscribe(Request::new(SubscribeRequest {
            app_id: app_id.to_string(),
            instance_id: instance_id.to_string(),
            replay: true,
        }))
        .await
        .unwrap();
    let mut stream = response.into_inner();

    // Expect ReplayStarted with the count of historical operations.
    let started = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(
        started.event,
        Some(SubscribeEventKind::ReplayStarted(lores_p2panda_server::proto::ReplayStarted {
            total_operations: 2,
        }))
    );

    // Collect the two historical operations.
    let event1 = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(operation_payload(&event1), b"first");

    let event2 = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(operation_payload(&event2), b"second");

    // Replay should finish before any live operations arrive.
    let ended = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(
        ended.event,
        Some(SubscribeEventKind::ReplayEnded(lores_p2panda_server::proto::ReplayEnded {}))
    );

    // Publish a new operation and verify it arrives as a live event on the same stream.
    service
        .publish(Request::new(PublishRequest {
            app_id: app_id.to_string(),
            instance_id: instance_id.to_string(),
            payload: b"third".to_vec(),
            idempotency_key: vec![],
        }))
        .await
        .unwrap();

    let event3 = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(operation_payload(&event3), b"third");
}

#[tokio::test]
async fn subscribe_with_replay_yields_replay_ended_without_live_operations() {
    let app_id = "test-app-empty";
    let instance_id = "test-instance-empty";
    let region_id = RegionId::generate();

    let (node_pool, _node_dir) = temp_db_pool("node-empty").await;
    let node_db_url = format!("{}", node_pool.connect_options().get_filename().display());
    let node = make_node(&node_db_url).await;
    let service = make_service(node.clone(), region_id).await;

    // Publish one operation so the replay has a single historical event and
    // then definitively ends, without waiting for any future live operation.
    service
        .publish(Request::new(PublishRequest {
            app_id: app_id.to_string(),
            instance_id: instance_id.to_string(),
            payload: b"only".to_vec(),
            idempotency_key: vec![],
        }))
        .await
        .unwrap();

    let response = service
        .subscribe(Request::new(SubscribeRequest {
            app_id: app_id.to_string(),
            instance_id: instance_id.to_string(),
            replay: true,
        }))
        .await
        .unwrap();
    let mut stream = response.into_inner();

    let started = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(
        started.event,
        Some(SubscribeEventKind::ReplayStarted(lores_p2panda_server::proto::ReplayStarted {
            total_operations: 1,
        }))
    );

    let event = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(operation_payload(&event), b"only");

    // Replay must end even though no live operations will ever be published.
    let ended = timeout(Duration::from_secs(30), stream.next()).await.unwrap().unwrap().unwrap();
    assert_eq!(
        ended.event,
        Some(SubscribeEventKind::ReplayEnded(lores_p2panda_server::proto::ReplayEnded {}))
    );
}

fn operation_payload(event: &SubscribeEvent) -> &[u8] {
    match &event.event {
        Some(SubscribeEventKind::Operation(op)) => &op.payload,
        other => panic!("expected operation event, got {:?}", other),
    }
}
