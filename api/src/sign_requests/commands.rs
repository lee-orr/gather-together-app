use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SignRequestCommand {
    Request {
        id: ulid::Ulid,
        contact: ulid::Ulid,
        sign_type: String,
        num_signs: usize,
        delivery_address: String,
        notes: Option<String>,
    },
    AdjustRequest {
        id: ulid::Ulid,
        contact: Option<ulid::Ulid>,
        sign_type: Option<String>,
        num_signs: Option<usize>,
        delivery_address: Option<String>,
        notes: Option<String>,
    },
    ConfirmDelivery {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_delivered: SignAmount,
    },
    RequestRemoval {
        id: ulid::Ulid,
        notes: Option<String>,
        num_signs: SignAmount,
    },
    ConfirmRemoval {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_removed: SignAmount,
    },
    MarkFailedDelivery {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_failed_delivery: SignAmount,
    },
    CancelRequest {
        id: ulid::Ulid,
    },
    MarkFailedRemoval {
        id: ulid::Ulid,
        agent: ulid::Ulid,
        notes: Option<String>,
        num_failed_retrieval: SignAmount,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SignAmount {
    All,
    None,
    Some(usize),
}
