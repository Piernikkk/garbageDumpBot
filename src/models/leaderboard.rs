use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Leaderboard {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub guild_id: i64,
    pub message: i64,
    pub channel_id: i64,
}
