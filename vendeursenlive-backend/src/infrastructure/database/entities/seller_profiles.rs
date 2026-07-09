use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "seller_profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub user_id: Uuid,
    pub shop_name: String,
    pub payment_link: Option<String>,
    pub trial_ends_at: DateTimeWithTimeZone,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(has_many = "super::live_sessions::Entity")]
    LiveSessions,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::live_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LiveSessions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
