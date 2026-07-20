# noctyrn-shared

Shared type definitions and protocol for Noctyrn. This crate is a dependency of both `noctyrn-game` (client) and `noctyrn-server` (server), ensuring they agree on data formats for network communication.

## What's in it

```
src/
├── lib.rs       # Module declarations
├── protocol.rs  # Network protocol types: ClientMessage, ServerMessage,
│                #   PlayerInput, PlayerActions (bitflags), GameStateSnapshot,
│                #   PlayerState
├── player.rs    # Player data types: PlayerProfile, PlayerStats, FriendEntry,
│                #   FriendRequestInfo, FriendStatus, FriendRequestStatus
├── lobby.rs     # Lobby types: LobbyState, LobbyPlayer, LobbyStatus
├── gamemode.rs  # GameMode enum (FFA, TDM, CTF, etc.)
└── weapon.rs    # WeaponSkin, SkinRarity (Common through Legendary)
```

### Key types

- **`ClientMessage` / `ServerMessage`** - TCP message enums for lobby/matchmaking communication
- **`PlayerInput`** - Per-tick input sent from client to server (movement direction, look angles, actions)
- **`PlayerActions`** - Bitflag-style struct for compressed action state (jump, crouch, sprint, shoot, reload, interact, ability)
- **`GameStateSnapshot`** - Server-to-client game state broadcast (all player positions, health, etc.)
- **`PlayerProfile`** - Full player profile with stats, level, XP
- **`LobbyState`** - Lobby membership and readiness state
- **`GameMode`** - All supported game modes

### Design choices

- All types derive `Serialize` + `Deserialize` for JSON transport
- `PlayerActions` uses a manual bitflag pattern (`u8` with const bit masks) for compact wire representation
- `uuid::Uuid` used for all entity/player/lobby IDs
- No framework-specific dependencies - pure Rust + serde + uuid

## Setup

```bash
cd noctyrn-shared
nix develop
cargo check
```

## Dependencies

- `serde 1.0` - Serialization framework
- `serde_json 1.0` - JSON encoding/decoding
- `uuid 1.12` - Universally unique identifiers
