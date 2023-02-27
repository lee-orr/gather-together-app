use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use super::commands::SignAmount;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignRequestEvent {
    Requested {
        id: ulid::Ulid,
        contact: ulid::Ulid,
        sign_type: String,
        num_signs: usize,
        delivery_address: String,
        notes: Option<String>,
    },
    RequestAdjusted {
        id: ulid::Ulid,
        contact: Option<ulid::Ulid>,
        sign_type: Option<String>,
        num_signs: Option<usize>,
        delivery_address: Option<String>,
        notes: Option<String>,
    },
    Delivered {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_delivered: SignAmount,
    },
    RemovalRequested {
        id: ulid::Ulid,
        notes: Option<String>,
        num_signs: SignAmount,
    },
    Removed {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_removed: SignAmount,
    },
    DeliveryFailed {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_failed_delivery: SignAmount,
    },
    RequestCancelled {
        id: ulid::Ulid,
    },
    RemovalFailed {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_failed_retrieval: SignAmount,
    },
}

impl DomainEvent for SignRequestEvent {
    fn event_type(&self) -> String {
        match self {
            SignRequestEvent::Requested {
                id: _,
                contact: _,
                sign_type: _,
                num_signs: _,
                delivery_address: _,
                notes: _,
            } => "Requested",
            SignRequestEvent::RequestAdjusted {
                id: _,
                contact: _,
                sign_type: _,
                num_signs: _,
                delivery_address: _,
                notes: _,
            } => "RequestAdjusted",
            SignRequestEvent::Delivered {
                id: _,
                agent: _,
                notes: _,
                num_delivered: _,
            } => "Delivered",
            SignRequestEvent::RemovalRequested {
                id: _,
                notes: _,
                num_signs: _,
            } => "RemovalRequested",
            SignRequestEvent::Removed {
                id: _,
                agent: _,
                notes: _,
                num_removed: _,
            } => "Removed",
            SignRequestEvent::DeliveryFailed {
                id: _,
                agent: _,
                notes: _,
                num_failed_delivery: _,
            } => "DeliveryFailed",
            SignRequestEvent::RequestCancelled { id: _ } => "RequestCancelled",
            SignRequestEvent::RemovalFailed {
                id: _,
                agent: _,
                notes: _,
                num_failed_retrieval: _,
            } => "RemovalFailed",
        }
        .to_string()
    }

    fn event_version(&self) -> String {
        "0.1".to_string()
    }
}
