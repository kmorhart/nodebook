use std::{ops::Deref};

use sqlx::{FromRow, types::Json};
use serde::{ Serialize, Deserialize };
use uuid::Uuid;


#[derive(sqlx::Type, Debug, Clone, Copy, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct UserUuid(pub Uuid);

impl Deref for UserUuid {
    type Target = Uuid;

    fn deref(&self, ) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub ref_uuid: Uuid,
    pub user_uuid: UserUuid,

    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSettings {
    pub dark_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserWorkspaces {
    pub private: Vec<Uuid>,
    pub public: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserData {
    pub ref_uuid: Uuid,
    pub user_uuid: UserUuid,

    pub settings: Json<UserSettings>,
    pub workspaces: Json<UserWorkspaces>,
}