use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(50))")]
pub enum LiveSessionStatus {
    #[sea_orm(string_value = "Ongoing")]
    Ongoing,
    #[sea_orm(string_value = "Ended")]
    Ended,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "live_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub seller_profile_id: Uuid,
    pub tiktok_url: String,
    pub status: LiveSessionStatus,
    pub created_at: DateTimeWithTimeZone,
    pub ended_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::seller_profiles::Entity",
        from = "Column::SellerProfileId",
        to = "super::seller_profiles::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SellerProfile,
    #[sea_orm(has_many = "super::ephemeral_products::Entity")]
    EphemeralProducts,
}

impl Related<super::seller_profiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SellerProfile.def()
    }
}

impl Related<super::ephemeral_products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EphemeralProducts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
