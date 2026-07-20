use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Full player profile returned from the server.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub level: i32,
    pub xp: i32,
    pub stats: PlayerStats,
    pub currency: i64,
    /// ISO 8601 formatted timestamp.
    pub created_at: String,
}

/// Aggregate stats tracked across all matches.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PlayerStats {
    pub total_kills: i32,
    pub total_deaths: i32,
    pub total_wins: i32,
    pub total_losses: i32,
    pub total_matches: i32,
    pub playtime_seconds: i64,
}

impl PlayerStats {
    /// Kill/death ratio, returns 0.0 if no deaths recorded.
    pub fn kd_ratio(&self) -> f64 {
        if self.total_deaths == 0 {
            self.total_kills as f64
        } else {
            self.total_kills as f64 / self.total_deaths as f64
        }
    }

    /// Win rate as a percentage (0.0 - 100.0), returns 0.0 if no matches played.
    pub fn win_rate(&self) -> f64 {
        if self.total_matches == 0 {
            0.0
        } else {
            (self.total_wins as f64 / self.total_matches as f64) * 100.0
        }
    }
}

/// A single entry in the player's friends list.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FriendEntry {
    pub id: Uuid,
    pub username: String,
    pub level: i32,
    pub online: bool,
}

/// Information about a pending friend request (incoming or outgoing).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FriendRequestInfo {
    pub id: Uuid,
    pub from_username: String,
    pub from_user_id: Uuid,
    pub to_username: String,
    pub to_user_id: Uuid,
    /// ISO 8601 formatted timestamp.
    pub created_at: String,
}
