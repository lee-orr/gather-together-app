use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use super::{
    commands::SignRequestCommand, errors::SignRequestError, events::SignRequestEvent,
    services::SignRequestServices,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SignRequest {
    contact: ulid::Ulid,
    id: ulid::Ulid,
    delivery_address: String,
    sign_type: String,
    notes: Vec<String>,
    num_signs_requested: usize,
    num_signs_delivered: usize,
    num_signs_retrieved: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Requested,
    Delivered,
    RemovalRequested,
    Removed,
    FailedDelivery,
    FailedRemoval,
}

impl Default for Status {
    fn default() -> Self {
        Self::Requested
    }
}

#[async_trait]
impl Aggregate for SignRequest {
    type Command = SignRequestCommand;
    type Error = SignRequestError;
    type Event = SignRequestEvent;
    type Services = SignRequestServices;

    fn aggregate_type() -> String {
        "SignRequest".to_string()
    }

    async fn handle(
        &self,
        _command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, _event: Self::Event) {
        todo!()
    }
}
