#[derive(Debug, Serialize, Deserialize)]
pub enum SignRequestCommand {
    RequestSigns {
        pub id: ulid::Ulid,
        pub contact: ulid::Ulid,
        pub sign_type: String,
        pub num_signs: usize,
        pub delivery_address: String,
        pub notes: Option<String>
    },
    AdjustSignRequest {
        pub id: ulid::Ulid,
        pub contact: Option<ulid::Ulid>,
        pub sign_type: Option<String>,
        pub num_signs: Option<usize>,
        pub delivery_address: Option<String>,
        pub notes: Option<String>
    },
    ConfirmSignDelivery {
        pub id: ulid::Ulid,
        pub notes: Option<String>,
        pub num_delivered: SignAmount
    },
    RequestSignRemoval {
        pub id: ulid::Ulid,
        pub notes: Option<String>,
        pub num_signs: SignAmount
    },
    ConfirmSignRemoval {
        pub id: ulid::Ulid,
        pub notes: Option<String>,
        pub num_removed: SignAmount
    },
    MarkFailedDelivery {
        pub id: ulid::Ulid,
        pub notes: Option<String>,
        pub num_failed_delivery: SignAmount
    },
    CancelSignRequest {
        pub id: ulid::Ulid
    },
    MarkFailedRemoval {
        pub id: ulid::Ulid,
        pub notes: Option<String>,
        pub num_failed_retrieval: SignAmount
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignAmount {
    All,
    None,
    Some(usize)
}