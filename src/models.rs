use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use crate::schema::games;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct GameModel {
    pub id: String,
    pub name: String,
    pub creator: String,
    pub plays: i32,
    pub created_at: Option<String>,
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[diesel(table_name = games)]
pub struct NewGame {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub creator: String,
    pub plays: i32,
}