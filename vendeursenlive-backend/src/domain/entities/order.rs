use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::CustomerContact;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    ProofSubmitted,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_contact: CustomerContact,
    pub status: OrderStatus,
    pub proof_image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Order {
    pub fn create(
        product_id: Uuid,
        customer_id: Option<Uuid>,
        customer_contact: CustomerContact,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            product_id,
            customer_id,
            customer_contact,
            status: OrderStatus::Pending,
            proof_image_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn submit_payment_proof(&mut self, proof_image_url: String) {
        self.proof_image_url = Some(proof_image_url);
        self.status = OrderStatus::ProofSubmitted;
        self.updated_at = Utc::now();
    }

    pub fn accept(&mut self) {
        self.status = OrderStatus::Accepted;
        self.updated_at = Utc::now();
    }

    pub fn reject(&mut self) {
        self.status = OrderStatus::Rejected;
        self.updated_at = Utc::now();
    }
}
