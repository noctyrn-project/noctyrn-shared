use serde::{Deserialize, Serialize};

/// Weapon inventory slot categories.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WeaponSlot {
    Primary,
    Secondary,
    Melee,
    Equipment,
}

impl WeaponSlot {
    pub fn display_name(&self) -> &'static str {
        match self {
            WeaponSlot::Primary => "Primary",
            WeaponSlot::Secondary => "Secondary",
            WeaponSlot::Melee => "Melee",
            WeaponSlot::Equipment => "Equipment",
        }
    }
}

impl std::fmt::Display for WeaponSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Replicated weapon state sent over the network.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkWeaponState {
    pub weapon_id: String,
    pub slot: WeaponSlot,
    pub ammo_current: u32,
    pub ammo_reserve: u32,
    pub is_reloading: bool,
}
