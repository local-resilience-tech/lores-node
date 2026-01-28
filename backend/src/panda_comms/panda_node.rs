use p2panda_core::{Hash, PrivateKey, PublicKey};
use sqlx::SqlitePool;
use std::sync::Arc;
use thiserror::Error;

use crate::{
    api::auth_api::auth_backend::User,
    panda_comms::{lores_events::LoResEventPayload, network, panda_node_inner::PandaPublishError},
};

use super::panda_node_inner::PandaNodeInner;

#[derive(Debug, Error)]
pub enum PandaNodeError {
    #[error(transparent)]
    RuntimeStartup(#[from] std::io::Error),
    #[error(transparent)]
    RuntimeSpawn(#[from] tokio::task::JoinError),
    #[error(transparent)]
    NetworkError(#[from] network::NetworkError),
}

pub struct RequiredNodeParams {
    pub private_key: PrivateKey,
    pub network_id: Hash,
    pub bootstrap_node_id: Option<PublicKey>,
}

pub struct PandaNode {
    inner: Arc<PandaNodeInner>,
    #[allow(dead_code)]
    runtime: OwnedRuntimeOrHandle,
}

enum OwnedRuntimeOrHandle {
    Handle(tokio::runtime::Handle),
    OwnedRuntime(tokio::runtime::Runtime),
}

impl std::ops::Deref for OwnedRuntimeOrHandle {
    type Target = tokio::runtime::Handle;

    fn deref(&self) -> &Self::Target {
        match self {
            OwnedRuntimeOrHandle::Handle(handle) => handle,
            OwnedRuntimeOrHandle::OwnedRuntime(runtime) => runtime.handle(),
        }
    }
}

impl PandaNode {
    pub async fn new(
        params: &RequiredNodeParams,
        operations_pool: &SqlitePool,
    ) -> Result<Self, PandaNodeError> {
        let runtime = if let Ok(handle) = tokio::runtime::Handle::try_current() {
            OwnedRuntimeOrHandle::Handle(handle)
        } else {
            OwnedRuntimeOrHandle::OwnedRuntime(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()?,
            )
        };

        let network_id = params.network_id.clone();
        let private_key = params.private_key.clone();
        let bootstrap_node_id = params.bootstrap_node_id.clone();
        let operations_pool = operations_pool.clone();

        let inner = runtime
            .spawn(async move {
                PandaNodeInner::new(network_id, private_key, bootstrap_node_id, &operations_pool)
                    .await
            })
            .await??;

        Ok(PandaNode {
            inner: Arc::new(inner),
            runtime,
        })
    }

    pub async fn publish_persisted(
        &self,
        event_payload: LoResEventPayload,
        current_user: Option<User>,
    ) -> Result<(), PandaPublishError> {
        self.inner
            .publish_persisted(event_payload, current_user)
            .await
    }
}
