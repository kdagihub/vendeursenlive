use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ephemeral_products")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub live_session_id: Uuid,
    pub image_url: String,
    pub price_fcfa: i32,
    pub description: String,
    pub is_retained: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::live_sessions::Entity",
        from = "Column::LiveSessionId",
        to = "super::live_sessions::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    LiveSession,
    #[sea_orm(has_many = "super::orders::Entity")]
    Orders,
}

impl Related<super::live_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LiveSession.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
