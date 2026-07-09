use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(50))")]
pub enum UserStatus {
    #[sea_orm(string_value = "Active")]
    Active,
    #[sea_orm(string_value = "Disabled")]
    Disabled,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub is_admin: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_sessions::Entity")]
    AuthSessions,
    #[sea_orm(has_one = "super::customer_profiles::Entity")]
    CustomerProfile,
    #[sea_orm(has_many = "super::user_auth_identities::Entity")]
    AuthIdentities,
    #[sea_orm(has_one = "super::seller_profiles::Entity")]
    SellerProfile,
    #[sea_orm(has_many = "super::orders::Entity")]
    Orders,
}

impl Related<super::seller_profiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SellerProfile.def()
    }
}

impl Related<super::customer_profiles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomerProfile.def()
    }
}

impl Related<super::user_auth_identities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthIdentities.def()
    }
}

impl Related<super::auth_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthSessions.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
