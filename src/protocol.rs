use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::gamemode::GameMode;
use crate::lobby::{LobbyState, PartyInfo};
use crate::player::{FriendEntry, FriendRequestInfo, PlayerProfile};

// ---------------------------------------------------------------------------
// TCP Messages (reliable: auth, friends, lobby, matchmaking, chat)
// ---------------------------------------------------------------------------

/// Messages sent from client to server over TCP.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    // -- Auth --
    LoginRequest {
        email: String,
        password: String,
    },
    RegisterRequest {
        username: String,
        email: String,
        password: String,
    },
    /// Authenticate an existing TCP connection with a JWT token.
    /// Must be the first message sent after connecting.
    Authenticate {
        token: String,
    },

    // -- Friends --
    SendFriendRequest {
        target_username: String,
    },
    AcceptFriendRequest {
        request_id: Uuid,
    },
    DeclineFriendRequest {
        request_id: Uuid,
    },
    RemoveFriend {
        friend_id: Uuid,
    },
    GetFriendsList,
    GetFriendRequests,

    // -- Profile --
    GetProfile,

    // -- Lobby --
    CreateLobby {
        game_mode: GameMode,
    },
    JoinLobby {
        lobby_id: Uuid,
    },
    LeaveLobby,
    SetReady {
        ready: bool,
    },

    // -- Party --
    /// Invite another player to the party (by username).
    PartyInvite {
        username: String,
    },
    /// Accept a pending party invite.
    PartyAcceptInvite {
        party_id: Uuid,
    },
    /// Decline a pending party invite.
    PartyDeclineInvite {
        party_id: Uuid,
    },
    /// Leave the current party.
    PartyLeave,
    /// Kick a member from the party (leader only).
    PartyKick {
        member_id: Uuid,
    },
    /// Party leader creates a lobby that includes all party members.
    PartyCreateLobby {
        game_mode: GameMode,
    },
    /// Party leader starts matchmaking for the party.
    PartyStartSearch,

    // -- Matchmaking --
    QueueForMatch {
        game_mode: GameMode,
    },
    CancelMatchmaking,

    // -- Chat --
    ChatMessage {
        content: String,
    },

    // -- Gameplay --
    /// Player is ready to respawn (sent after the 5-second post-death delay).
    RequestRespawn,
}

/// Messages sent from server to client over TCP.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    // -- Auth responses --
    AuthSuccess {
        token: String,
        user_id: Uuid,
        username: String,
    },
    AuthError {
        message: String,
    },
    /// Confirmation that the TCP connection has been authenticated.
    Authenticated {
        user_id: Uuid,
        username: String,
    },

    // -- Profile --
    ProfileData {
        profile: PlayerProfile,
    },

    // -- Friends --
    FriendsList {
        friends: Vec<FriendEntry>,
    },
    FriendRequestsList {
        incoming: Vec<FriendRequestInfo>,
        outgoing: Vec<FriendRequestInfo>,
    },
    FriendRequestSent,
    FriendRequestAccepted,
    FriendRequestDeclined,
    FriendRemoved,
    FriendError {
        message: String,
    },

    // -- Lobby --
    LobbyUpdate {
        lobby: LobbyState,
    },
    LobbyError {
        message: String,
    },

    // -- Party --
    /// Push notification: someone invited you to a party.
    PartyInviteReceived {
        party_id: Uuid,
        from_username: String,
    },
    /// Push to all party members on any party state change.
    PartyUpdate {
        party: PartyInfo,
    },
    PartyError {
        message: String,
    },

    // -- Matchmaking --
    MatchmakingStatus {
        players_in_queue: u32,
        /// Estimated wait time in seconds, if available.
        estimated_wait: Option<u32>,
    },
    MatchFound {
        lobby_id: Uuid,
        server_addr: String,
        udp_port: u16,
    },

    // -- General --
    Error {
        message: String,
    },

    // -- Chat --
    ChatBroadcast {
        from_username: String,
        content: String,
    },
}

// ---------------------------------------------------------------------------
// UDP Messages (fast: gameplay synchronisation)
// ---------------------------------------------------------------------------

/// Bitflags representing discrete player actions packed into a single `u8`.
///
/// Multiple actions can be active in the same tick by OR-ing the flags together.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayerActions(pub u8);

impl PlayerActions {
    pub const JUMP: u8       = 1 << 0;
    pub const CROUCH: u8     = 1 << 1;
    pub const SPRINT: u8     = 1 << 2;
    pub const SHOOT: u8      = 1 << 3;
    pub const RELOAD: u8     = 1 << 4;
    pub const LEAN_LEFT: u8  = 1 << 5;
    pub const LEAN_RIGHT: u8 = 1 << 6;

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn contains(self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }

    #[inline]
    pub fn set(&mut self, flag: u8) {
        self.0 |= flag;
    }

    #[inline]
    pub fn clear(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    #[inline]
    pub fn set_if(&mut self, flag: u8, condition: bool) {
        if condition {
            self.set(flag);
        } else {
            self.clear(flag);
        }
    }
}

/// Dedicated shot-fired packet sent over UDP when the player fires.
///
/// Sent separately from `PlayerInput` so the aim origin/direction are captured
/// at the exact moment of firing, not when the next movement tick happens.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShotFired {
    /// Discriminator so the server can route this packet. Always `"ShotFired"`.
    #[serde(rename = "type")]
    pub packet_type: String,
    pub player_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    /// World-space origin of the shot (camera position).
    pub origin: [f32; 3],
    /// Normalised world-space aim direction.
    pub direction: [f32; 3],
    /// Weapon identifier (e.g. "colt_m4a1").
    pub weapon_id: String,
    /// Client timestamp when the shot was fired (for lag compensation).
    pub timestamp: f64,
}

impl ShotFired {
    pub fn new(player_id: uuid::Uuid, session_id: uuid::Uuid, origin: [f32; 3], direction: [f32; 3], weapon_id: String, timestamp: f64) -> Self {
        Self {
            packet_type: "ShotFired".to_string(),
            player_id,
            session_id,
            origin,
            direction,
            weapon_id,
            timestamp,
        }
    }
}

/// Client-to-server input packet sent every client tick over UDP.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInput {
    /// Monotonically increasing sequence number for this input.
    pub sequence: u32,
    /// Client-local timestamp (seconds) when the input was sampled.
    pub timestamp: f64,
    /// The game session this input belongs to (set once, used for address registration).
    pub session_id: uuid::Uuid,
    /// The sending player's id (set once, used for address registration).
    pub player_id: uuid::Uuid,
    /// Normalised wish-direction vector `[x, y, z]`.
    pub movement: [f32; 3],
    /// Camera yaw in radians.
    pub look_yaw: f32,
    /// Camera pitch in radians.
    pub look_pitch: f32,
    /// Packed action flags for discrete actions (jump, shoot, etc.).
    pub actions: PlayerActions,
}

/// Server-to-client authoritative game state snapshot broadcast every server tick over UDP.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameStateSnapshot {
    /// Server tick number.
    pub tick: u64,
    /// Server wall-clock time in seconds.
    pub server_time: f64,
    /// The sequence number of the last `PlayerInput` the server has processed
    /// for the receiving client (used for client-side reconciliation).
    pub last_processed_input: u32,
    /// Authoritative state for every player in the match.
    pub players: Vec<NetworkPlayerState>,
    /// Events that occurred during this tick.
    pub events: Vec<GameEvent>,
}

/// Replicated per-player state included in every snapshot.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkPlayerState {
    pub id: Uuid,
    /// Player display name.
    pub username: String,
    /// World position `[x, y, z]`.
    pub position: [f32; 3],
    /// Velocity vector `[x, y, z]`.
    pub velocity: [f32; 3],
    /// Camera / body yaw in radians.
    pub yaw: f32,
    /// Camera pitch in radians.
    pub pitch: f32,
    /// Current health points.
    pub health: f32,
    /// Identifier for the weapon currently held.
    pub weapon_id: String,
    /// Encoded movement state (standing, crouching, sprinting, etc.).
    pub movement_state: u8,
}

/// Discrete in-game events attached to a snapshot tick.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum GameEvent {
    PlayerKilled {
        killer_id: Uuid,
        victim_id: Uuid,
        weapon: String,
    },
    PlayerDamaged {
        target_id: Uuid,
        damage: f32,
        source_id: Uuid,
    },
    PlayerRespawned {
        player_id: Uuid,
        position: [f32; 3],
    },
    ProjectileFired {
        owner_id: Uuid,
        origin: [f32; 3],
        direction: [f32; 3],
        weapon: String,
    },
    MatchStateUpdate {
        time_remaining: f32,
        scores: Vec<(Uuid, i32)>,
    },
}

// ---------------------------------------------------------------------------
// Serialisation / deserialisation helpers
// ---------------------------------------------------------------------------
//
// TCP uses **length-prefixed JSON**: a 4-byte big-endian `u32` length header
// followed by that many bytes of UTF-8 JSON.
//
// UDP also uses JSON for now.  We can swap to a compact binary encoding
// (e.g. bincode / MessagePack) later without changing the public API -- only
// these helper functions need updating.
// ---------------------------------------------------------------------------

/// Serialise a `ClientMessage` into a length-prefixed JSON byte buffer (TCP framing).
pub fn encode_client_message(msg: &ClientMessage) -> Result<Vec<u8>, serde_json::Error> {
    encode_length_prefixed(msg)
}

/// Deserialise a `ClientMessage` from a length-prefixed JSON byte buffer.
///
/// `data` must start with the 4-byte length header.
pub fn decode_client_message(data: &[u8]) -> Result<ClientMessage, DecodeError> {
    decode_length_prefixed(data)
}

/// Serialise a `ServerMessage` into a length-prefixed JSON byte buffer (TCP framing).
pub fn encode_server_message(msg: &ServerMessage) -> Result<Vec<u8>, serde_json::Error> {
    encode_length_prefixed(msg)
}

/// Deserialise a `ServerMessage` from a length-prefixed JSON byte buffer.
pub fn decode_server_message(data: &[u8]) -> Result<ServerMessage, DecodeError> {
    decode_length_prefixed(data)
}

/// Serialise a `PlayerInput` to JSON bytes (UDP payload).
pub fn encode_player_input(input: &PlayerInput) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(input)
}

/// Deserialise a `PlayerInput` from JSON bytes (UDP payload).
pub fn decode_player_input(data: &[u8]) -> Result<PlayerInput, serde_json::Error> {
    serde_json::from_slice(data)
}

/// Serialise a `ShotFired` into raw JSON bytes (UDP framing).
pub fn encode_shot_fired(shot: &ShotFired) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(shot)
}

/// Deserialise a `ShotFired` from raw JSON bytes.
pub fn decode_shot_fired(data: &[u8]) -> Result<ShotFired, serde_json::Error> {
    serde_json::from_slice(data)
}

/// Serialise a `GameStateSnapshot` into raw JSON bytes (UDP framing).
pub fn encode_game_state(snapshot: &GameStateSnapshot) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(snapshot)
}

/// Deserialise a `GameStateSnapshot` from JSON bytes (UDP payload).
pub fn decode_game_state(data: &[u8]) -> Result<GameStateSnapshot, serde_json::Error> {
    serde_json::from_slice(data)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Errors that can occur when decoding a length-prefixed message.
#[derive(Debug)]
pub enum DecodeError {
    /// The buffer is shorter than the 4-byte length header.
    InsufficientData,
    /// The buffer contains fewer bytes than the length header promises.
    Truncated { expected: usize, actual: usize },
    /// JSON deserialisation failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::InsufficientData => write!(f, "insufficient data for length header"),
            DecodeError::Truncated { expected, actual } => {
                write!(f, "truncated message: expected {expected} bytes, got {actual}")
            }
            DecodeError::Json(e) => write!(f, "json decode error: {e}"),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodeError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for DecodeError {
    fn from(e: serde_json::Error) -> Self {
        DecodeError::Json(e)
    }
}

/// Encode any serialisable value as `[4-byte big-endian length][json bytes]`.
fn encode_length_prefixed<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    let json = serde_json::to_vec(value)?;
    let len = json.len() as u32;
    let mut buf = Vec::with_capacity(4 + json.len());
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(&json);
    Ok(buf)
}

/// Decode a length-prefixed JSON message.  Returns the deserialised value.
fn decode_length_prefixed<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, DecodeError> {
    if data.len() < 4 {
        return Err(DecodeError::InsufficientData);
    }
    let len = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let payload = &data[4..];
    if payload.len() < len {
        return Err(DecodeError::Truncated {
            expected: len,
            actual: payload.len(),
        });
    }
    let value = serde_json::from_slice(&payload[..len])?;
    Ok(value)
}

// ---------------------------------------------------------------------------
// TCP stream framing helpers
// ---------------------------------------------------------------------------

/// Read the length header from the first 4 bytes of a buffer.
///
/// Returns `None` if the buffer is shorter than 4 bytes.
pub fn read_message_length(buf: &[u8]) -> Option<u32> {
    if buf.len() < 4 {
        return None;
    }
    Some(u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]))
}

/// Total number of bytes needed for one complete length-prefixed message
/// (including the 4 header bytes), given the payload length from `read_message_length`.
pub fn total_frame_size(payload_len: u32) -> usize {
    4 + payload_len as usize
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_client_message() {
        let msg = ClientMessage::LoginRequest {
            email: "test@example.com".into(),
            password: "hunter2".into(),
        };
        let encoded = encode_client_message(&msg).unwrap();
        let decoded: ClientMessage = decode_client_message(&encoded).unwrap();
        // Verify the round-trip by re-encoding and comparing bytes.
        let re_encoded = encode_client_message(&decoded).unwrap();
        assert_eq!(encoded, re_encoded);
    }

    #[test]
    fn roundtrip_server_message() {
        let msg = ServerMessage::AuthSuccess {
            token: "jwt-token-here".into(),
            user_id: Uuid::new_v4(),
            username: "player1".into(),
        };
        let encoded = encode_server_message(&msg).unwrap();
        let decoded: ServerMessage = decode_server_message(&encoded).unwrap();
        let re_encoded = encode_server_message(&decoded).unwrap();
        assert_eq!(encoded, re_encoded);
    }

    #[test]
    fn roundtrip_player_input() {
        let mut actions = PlayerActions::empty();
        actions.set(PlayerActions::JUMP);
        actions.set(PlayerActions::SHOOT);

        let input = PlayerInput {
            sequence: 42,
            timestamp: 1.234,
            session_id: Uuid::default(),
            player_id: Uuid::default(),
            movement: [0.0, 0.0, 1.0],
            look_yaw: 1.57,
            look_pitch: -0.3,
            actions,
        };
        let encoded = encode_player_input(&input).unwrap();
        let decoded = decode_player_input(&encoded).unwrap();
        assert_eq!(decoded.sequence, 42);
        assert!(decoded.actions.contains(PlayerActions::JUMP));
        assert!(decoded.actions.contains(PlayerActions::SHOOT));
        assert!(!decoded.actions.contains(PlayerActions::CROUCH));
    }

    #[test]
    fn roundtrip_game_state_snapshot() {
        let snapshot = GameStateSnapshot {
            tick: 100,
            server_time: 5.0,
            last_processed_input: 40,
            players: vec![NetworkPlayerState {
                id: Uuid::new_v4(),
                position: [1.0, 2.0, 3.0],
                velocity: [0.0, 0.0, 0.0],
                yaw: 0.0,
                pitch: 0.0,
                health: 100.0,
                weapon_id: "ak47".into(),
                movement_state: 0,
            }],
            events: vec![GameEvent::PlayerKilled {
                killer_id: Uuid::new_v4(),
                victim_id: Uuid::new_v4(),
                weapon: "ak47".into(),
            }],
        };
        let encoded = encode_game_state(&snapshot).unwrap();
        let decoded = decode_game_state(&encoded).unwrap();
        assert_eq!(decoded.tick, 100);
        assert_eq!(decoded.players.len(), 1);
        assert_eq!(decoded.events.len(), 1);
    }

    #[test]
    fn player_actions_bitflags() {
        let mut a = PlayerActions::empty();
        assert!(!a.contains(PlayerActions::JUMP));

        a.set(PlayerActions::JUMP);
        assert!(a.contains(PlayerActions::JUMP));

        a.set(PlayerActions::SPRINT);
        assert!(a.contains(PlayerActions::JUMP));
        assert!(a.contains(PlayerActions::SPRINT));

        a.clear(PlayerActions::JUMP);
        assert!(!a.contains(PlayerActions::JUMP));
        assert!(a.contains(PlayerActions::SPRINT));

        a.set_if(PlayerActions::CROUCH, true);
        assert!(a.contains(PlayerActions::CROUCH));
        a.set_if(PlayerActions::CROUCH, false);
        assert!(!a.contains(PlayerActions::CROUCH));
    }

    #[test]
    fn decode_error_insufficient_data() {
        let result = decode_client_message(&[0u8; 2]);
        assert!(matches!(result, Err(DecodeError::InsufficientData)));
    }

    #[test]
    fn decode_error_truncated() {
        // Length header says 100 bytes, but we only supply 4.
        let mut data = vec![0, 0, 0, 100];
        data.extend_from_slice(b"{}");
        let result = decode_client_message(&data);
        assert!(matches!(result, Err(DecodeError::Truncated { .. })));
    }
}
