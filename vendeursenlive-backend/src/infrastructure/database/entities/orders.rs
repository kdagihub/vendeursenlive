use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(50))")]
pub enum OrderStatus {
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "ProofSubmitted")]
    ProofSubmitted,
    #[sea_orm(string_value = "Accepted")]
    Accepted,
    #[sea_orm(string_value = "Rejected")]
    Rejected,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_name: String,
    pub customer_phone: String,
    pub customer_location: String,
    pub status: OrderStatus,
    pub proof_image_url: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ephemeral_products::Entity",
        from = "Column::ProductId",
        to = "super::ephemeral_products::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Product,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CustomerId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Customer,
}

impl Related<super::ephemeral_products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Product.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
