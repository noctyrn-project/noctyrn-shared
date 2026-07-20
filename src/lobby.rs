use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::gamemode::GameMode;

/// Full lobby state broadcasted to all lobby members on any change.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyState {
    pub id: Uuid,
    pub game_mode: GameMode,
    pub players: Vec<LobbyPlayer>,
    pub max_players: u32,
    pub state: LobbyStatus,
}

impl LobbyState {
    /// Whether the lobby has room for more players.
    pub fn is_full(&self) -> bool {
        self.players.len() >= self.max_players as usize
    }

    /// Number of players currently in the lobby.
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Whether all players in the lobby are ready.
    pub fn all_ready(&self) -> bool {
        !self.players.is_empty() && self.players.iter().all(|p| p.ready)
    }
}

/// Current phase of the lobby lifecycle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LobbyStatus {
    /// Waiting for players to join and ready up.
    Waiting,
    /// Countdown has started, match is about to begin.
    Starting,
    /// Match is actively in progress.
    InGame,
}

/// A player slot inside a lobby.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyPlayer {
    pub id: Uuid,
    pub username: String,
    pub ready: bool,
    pub team: Option<String>,
}

/// Current phase of the party lifecycle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PartyStatus {
    /// Party is formed but idle (no lobby, no matchmaking).
    Idle,
    /// Party members are in a lobby, waiting to ready up.
    InLobby,
    /// Party is in matchmaking search.
    Searching,
    /// Party is actively in a game session.
    InGame,
}

/// Party (pre-lobby group) information.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartyInfo {
    pub id: Uuid,
    pub leader_id: Uuid,
    pub members: Vec<PartyMember>,
    pub status: PartyStatus,
}

impl PartyInfo {
    /// Whether the given user is the party leader.
    pub fn is_leader(&self, user_id: Uuid) -> bool {
        self.leader_id == user_id
    }

    /// Number of members in the party.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// A member of a party.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartyMember {
    pub id: Uuid,
    pub username: String,
}
