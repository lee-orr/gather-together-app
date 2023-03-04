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
    status: Status,
    delivery_address: String,
    sign_type: String,
    notes: Vec<String>,
    num_signs_requested: usize,
    num_signs_delivered: usize,
    num_signs_retrieved: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Uninitialized,
    Requested,
    Delivered,
    RemovalRequested,
    Removed,
    FailedDelivery,
    FailedRemoval,
}

impl Default for Status {
    fn default() -> Self {
        Self::Uninitialized
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
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            SignRequestCommand::Request {
                id,
                contact,
                sign_type,
                num_signs,
                delivery_address,
                notes,
            } => {
                if self.status == Status::Uninitialized {
                    Ok(vec![SignRequestEvent::Requested {
                        id,
                        contact,
                        sign_type,
                        num_signs,
                        delivery_address,
                        notes,
                    }])
                } else {
                    Err("Sign request exists already".into())
                }
            }
            SignRequestCommand::AdjustRequest {
                id,
                contact,
                sign_type,
                num_signs,
                delivery_address,
                notes,
            } => {
                if self.status != Status::Uninitialized {
                    Ok(vec![SignRequestEvent::RequestAdjusted {
                        id,
                        contact,
                        sign_type,
                        num_signs,
                        delivery_address,
                        notes,
                    }])
                } else {
                    Err("Sign request exists already".into())
                }
            }
            SignRequestCommand::ConfirmDelivery {
                id: _,
                agent: _,
                notes: _,
                num_delivered: _,
            } => todo!(),
            SignRequestCommand::RequestRemoval {
                id: _,
                notes: _,
                num_signs: _,
            } => todo!(),
            SignRequestCommand::ConfirmRemoval {
                id: _,
                agent: _,
                notes: _,
                num_removed: _,
            } => todo!(),
            SignRequestCommand::MarkFailedDelivery {
                id: _,
                agent: _,
                notes: _,
                num_failed_delivery: _,
            } => todo!(),
            SignRequestCommand::CancelRequest { id: _ } => todo!(),
            SignRequestCommand::MarkFailedRemoval {
                id: _,
                agent: _,
                notes: _,
                num_failed_retrieval: _,
            } => todo!(),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            SignRequestEvent::Requested {
                id,
                contact,
                sign_type,
                num_signs,
                delivery_address,
                notes,
            } => {
                self.id = id;
                self.contact = contact;
                self.sign_type = sign_type;
                self.num_signs_requested = num_signs;
                self.delivery_address = delivery_address;
                self.notes = vec![];
                self.status = Status::Requested;
                if let Some(notes) = notes {
                    self.notes.push(notes);
                }
            }
            SignRequestEvent::RequestAdjusted {
                id: _,
                contact,
                sign_type,
                num_signs,
                delivery_address,
                notes,
            } => {
                if let Some(contact) = contact {
                    self.contact = contact;
                }
                if let Some(sign_type) = sign_type {
                    self.sign_type = sign_type;
                }
                if let Some(num_signs) = num_signs {
                    self.num_signs_requested = num_signs;
                }
                if let Some(delivery_address) = delivery_address {
                    self.delivery_address = delivery_address;
                }
                if let Some(notes) = notes {
                    self.notes.push(notes);
                }
            }
            SignRequestEvent::Delivered {
                id: _,
                agent: _,
                notes: _,
                num_delivered: _,
            } => todo!(),
            SignRequestEvent::RemovalRequested {
                id: _,
                notes: _,
                num_signs: _,
            } => todo!(),
            SignRequestEvent::Removed {
                id: _,
                agent: _,
                notes: _,
                num_removed: _,
            } => todo!(),
            SignRequestEvent::DeliveryFailed {
                id: _,
                agent: _,
                notes: _,
                num_failed_delivery: _,
            } => todo!(),
            SignRequestEvent::RequestCancelled { id: _ } => todo!(),
            SignRequestEvent::RemovalFailed {
                id: _,
                agent: _,
                notes: _,
                num_failed_retrieval: _,
            } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cqrs_es::test::TestFramework;

    type Framework = TestFramework<SignRequest>;

    #[test]
    fn test_request_signs() {
        let id = ulid::Ulid::from_parts(1, 1);
        let contact = ulid::Ulid::from_parts(1, 2);
        let expected = SignRequestEvent::Requested {
            id,
            contact,
            sign_type: "sign".to_string(),
            num_signs: 1,
            delivery_address: "my address".to_string(),
            notes: None,
        };

        Framework::with(SignRequestServices)
            .given_no_previous_events()
            .when(SignRequestCommand::Request {
                id,
                contact,
                sign_type: "sign".to_string(),
                num_signs: 1,
                delivery_address: "my address".to_string(),
                notes: None,
            })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_cannot_recreate_existing_request() {
        let id = ulid::Ulid::from_parts(1, 1);
        let contact = ulid::Ulid::from_parts(1, 2);
        let expected = SignRequestEvent::Requested {
            id,
            contact,
            sign_type: "sign".to_string(),
            num_signs: 1,
            delivery_address: "my address".to_string(),
            notes: None,
        };

        Framework::with(SignRequestServices)
            .given(vec![expected])
            .when(SignRequestCommand::Request {
                id,
                contact,
                sign_type: "sign".to_string(),
                num_signs: 1,
                delivery_address: "my address".to_string(),
                notes: None,
            })
            .then_expect_error_message("Sign request exists already");
    }

    #[test]
    fn test_adjust_request() {
        let id = ulid::Ulid::from_parts(1, 1);
        let contact = ulid::Ulid::from_parts(1, 2);
        let previous = SignRequestEvent::Requested {
            id,
            contact,
            sign_type: "sign".to_string(),
            num_signs: 1,
            delivery_address: "my address".to_string(),
            notes: None,
        };

        let expected = SignRequestEvent::RequestAdjusted {
            id,
            contact: None,
            sign_type: None,
            num_signs: Some(5),
            delivery_address: None,
            notes: None,
        };

        Framework::with(SignRequestServices)
            .given(vec![previous])
            .when(SignRequestCommand::AdjustRequest {
                id,
                contact: None,
                sign_type: None,
                num_signs: Some(5),
                delivery_address: None,
                notes: None,
            })
            .then_expect_events(vec![expected]);
    }
}
