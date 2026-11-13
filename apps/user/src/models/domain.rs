use crate::models::db::{ UserUuid }; 
use serde::{ Serialize, Deserialize };


// Internal structs for actions describing user intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistration {
    pub uuid: UserUuid,
    pub username: String,
}


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UserProfilePublic {
//     pub uuid: UserUuid,
//     pub username: String,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub bio: Option<String>,
// }