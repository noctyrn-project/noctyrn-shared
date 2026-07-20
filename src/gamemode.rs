use serde::{Deserialize, Serialize};

/// All supported game modes, shared between client and server.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameMode {
    FreeForAll,
    TeamDeathmatch,
    KillConfirmed,
    CaptureTheFlag,
    Assassins,
    KingOfTheHill,
    Hardpoint,
    CapturePoint,
    TestingGrounds,
    Juggernaut,
    HighExplosives,
    OneInTheChamber,
    GunGame,
    Infected,
}

impl GameMode {
    /// Human-readable display name for UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            GameMode::FreeForAll => "Free For All",
            GameMode::TeamDeathmatch => "Team Deathmatch",
            GameMode::KillConfirmed => "Kill Confirmed",
            GameMode::CaptureTheFlag => "Capture The Flag",
            GameMode::Assassins => "Assassins",
            GameMode::KingOfTheHill => "King of the Hill",
            GameMode::Hardpoint => "Hardpoint",
            GameMode::CapturePoint => "Capture Point",
            GameMode::TestingGrounds => "Testing Grounds",
            GameMode::Juggernaut => "Juggernaut",
            GameMode::HighExplosives => "High Explosives",
            GameMode::OneInTheChamber => "One in the Chamber",
            GameMode::GunGame => "Gun Game",
            GameMode::Infected => "Infected",
        }
    }

    /// Short abbreviated name for scoreboards, HUD elements, etc.
    pub fn short_name(&self) -> &'static str {
        match self {
            GameMode::FreeForAll => "FFA",
            GameMode::TeamDeathmatch => "TDM",
            GameMode::KillConfirmed => "KC",
            GameMode::CaptureTheFlag => "CTF",
            GameMode::Assassins => "ASN",
            GameMode::KingOfTheHill => "KOTH",
            GameMode::Hardpoint => "HP",
            GameMode::CapturePoint => "CP",
            GameMode::TestingGrounds => "TG",
            GameMode::Juggernaut => "JUG",
            GameMode::HighExplosives => "HE",
            GameMode::OneInTheChamber => "OITC",
            GameMode::GunGame => "GG",
            GameMode::Infected => "INF",
        }
    }

    /// Maximum number of players allowed in a match of this mode.
    pub fn max_players(&self) -> u32 {
        match self {
            GameMode::FreeForAll => 12,
            GameMode::TeamDeathmatch => 12,
            GameMode::KillConfirmed => 12,
            GameMode::CaptureTheFlag => 12,
            GameMode::Assassins => 10,
            GameMode::KingOfTheHill => 12,
            GameMode::Hardpoint => 12,
            GameMode::CapturePoint => 12,
            GameMode::TestingGrounds => 16,
            GameMode::Juggernaut => 10,
            GameMode::HighExplosives => 12,
            GameMode::OneInTheChamber => 8,
            GameMode::GunGame => 10,
            GameMode::Infected => 14,
        }
    }

    /// Minimum number of players required to start a match.
    pub fn min_players(&self) -> u32 {
        match self {
            GameMode::FreeForAll => 2,
            GameMode::TeamDeathmatch => 4,
            GameMode::KillConfirmed => 4,
            GameMode::CaptureTheFlag => 4,
            GameMode::Assassins => 3,
            GameMode::KingOfTheHill => 4,
            GameMode::Hardpoint => 4,
            GameMode::CapturePoint => 4,
            GameMode::TestingGrounds => 1,
            GameMode::Juggernaut => 3,
            GameMode::HighExplosives => 4,
            GameMode::OneInTheChamber => 2,
            GameMode::GunGame => 2,
            GameMode::Infected => 4,
        }
    }

    /// Whether this mode uses team-based gameplay.
    pub fn is_team_based(&self) -> bool {
        matches!(
            self,
            GameMode::TeamDeathmatch
                | GameMode::KillConfirmed
                | GameMode::CaptureTheFlag
                | GameMode::KingOfTheHill
                | GameMode::Hardpoint
                | GameMode::CapturePoint
                | GameMode::HighExplosives
                | GameMode::Infected
        )
    }

    /// Number of teams for team-based modes (0 for non-team modes).
    pub fn team_count(&self) -> u32 {
        match self {
            _ if !self.is_team_based() => 0,
            GameMode::Infected => 2, // survivors vs infected
            _ => 2,
        }
    }

    /// Iterator over all game mode variants.
    pub fn all() -> &'static [GameMode] {
        &[
            GameMode::FreeForAll,
            GameMode::TeamDeathmatch,
            GameMode::KillConfirmed,
            GameMode::CaptureTheFlag,
            GameMode::Assassins,
            GameMode::KingOfTheHill,
            GameMode::Hardpoint,
            GameMode::CapturePoint,
            GameMode::TestingGrounds,
            GameMode::Juggernaut,
            GameMode::HighExplosives,
            GameMode::OneInTheChamber,
            GameMode::GunGame,
            GameMode::Infected,
        ]
    }
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
