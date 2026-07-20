pub mod protocol;
pub mod player;
pub mod lobby;
pub mod gamemode;
pub mod weapon;

// Re-export commonly used types at crate root for convenience.
pub use gamemode::GameMode;
pub use player::{PlayerProfile, PlayerStats, FriendEntry, FriendRequestInfo};
pub use lobby::{LobbyState, LobbyStatus, LobbyPlayer, PartyInfo, PartyMember, PartyStatus};
pub use weapon::{WeaponSlot, NetworkWeaponState};
pub use protocol::{
    ClientMessage, ServerMessage,
    PlayerInput, PlayerActions,
    GameStateSnapshot, NetworkPlayerState, GameEvent,
};
