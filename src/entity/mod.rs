
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub is_admin: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostIdent {

    // Post ID
    pub id: u32,

    // Post Title
    pub title: String,

    // Timestamp when it was created
    // (ms since Unix epoch - but only accurate to the second)
    pub created: i64,
}
