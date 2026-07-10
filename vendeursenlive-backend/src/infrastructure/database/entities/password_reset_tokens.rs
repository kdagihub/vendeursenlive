use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "password_reset_tokens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_auth_identity_id: Uuid,
    pub token_hash: String,
    pub used_at: Option<DateTimeWithTimeZone>,
    pub expires_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_auth_identities::Entity",
        from = "Column::UserAuthIdentityId",
        to = "super::user_auth_identities::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    UserAuthIdentity,
}

impl Related<super::user_auth_identities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserAuthIdentity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
