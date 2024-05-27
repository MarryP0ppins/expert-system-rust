//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::{entity::prelude::*, ActiveValue::NotSet, IntoActiveModel, Set, Unchanged};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel, Eq, ToSchema)]
#[schema(as = UserModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    #[schema(read_only)]
    pub id: i32,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub username: String,
    #[serde(skip_deserializing)]
    pub created_at: DateTime,
    pub first_name: String,
    pub last_name: String,
    pub is_superuser: bool,
    #[serde(skip_serializing)]
    pub password: String,
}

pub use Model as UserModel;

#[derive(Clone, Debug, Serialize, Deserialize, DeriveIntoActiveModel, ToSchema)]
pub struct LoginUserModel {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UpdateUserResponse {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub new_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserModel {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::histories::Entity")]
    Histories,
    #[sea_orm(has_many = "super::systems::Entity")]
    Systems,
}

impl Related<super::histories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Histories.def()
    }
}

impl Related<super::systems::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Systems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl IntoActiveModel<ActiveModel> for UpdateUserModel {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Unchanged(self.id),
            email: self.email.map_or(NotSet, |email| Set(email)),
            first_name: self.first_name.map_or(NotSet, |first_name| Set(first_name)),
            last_name: self.last_name.map_or(NotSet, |last_name| Set(last_name)),
            password: self.password.map_or(NotSet, |password| Set(password)),
            ..Default::default()
        }
    }
}
