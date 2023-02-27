use serde::{Serialize, Deserialize};

use super::commands::SignAmount;


#[derive(Debug, Serialize, Deserialize)]
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