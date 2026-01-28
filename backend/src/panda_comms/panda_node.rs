use std::sync::Arc;
use thiserror::Error;

use super::panda_node_inner::PandaNodeInner;

#[derive(Debug, Error)]
pub enum PandaNodeError {
    #[error(transparent)]
    RuntimeStartup(#[from] std::io::Error),
    #[error(transparent)]
    RuntimeSpawn(#[from] tokio::task::JoinError),
}

pub struct PandaNode {
    inner: Arc<PandaNodeInner>,
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
    pub async fn new() -> Result<Self, PandaNodeError> {
        let runtime = if let Ok(handle) = tokio::runtime::Handle::try_current() {
            OwnedRuntimeOrHandle::Handle(handle)
        } else {
            OwnedRuntimeOrHandle::OwnedRuntime(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()?,
            )
        };

        let inner = runtime
            .spawn(async move { PandaNodeInner::new().await })
            .await??;

        Ok(PandaNode {
            inner: Arc::new(inner),
        })
    }
}
