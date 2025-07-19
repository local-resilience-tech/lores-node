use crate::admin_api::client_events::ClientEvent;

#[derive(Default)]
pub struct HandlerResult {
    pub client_events: Vec<ClientEvent>,
}
