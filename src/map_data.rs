use serde::Deserialize;

/// An oriented bounding-box collider, stored in the mesh's local
/// coordinate space (pre-scale). Apply the map's `scale` factor at runtime.
/// Each entry corresponds to a single triangle face, with `half_extents.z`
/// set to a small fixed thickness along the face normal.
#[derive(Deserialize, Clone, Debug)]
pub struct ColliderBox {
    pub center: [f32; 3],
    pub half_extents: [f32; 3],
    /// Rotation quaternion (w, x, y, z) that orients the box from its
    /// axis-aligned rest pose to the face's local frame.
    #[serde(default = "default_identity")]
    pub rotation: [f32; 4],
    /// Material type for bullet penetration, footstep sounds, etc.
    /// 0 = Concrete, 1 = Metal, 2 = Wood, 3 = Glass, 4 = Drywall
    pub material: u8,
}

fn default_identity() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

/// Per-map spawn point and objective data, shared by server and client.
#[derive(Deserialize, Clone, Debug)]
pub struct MapData {
    /// World-space spawn positions (feet-level, Y = ground height).
    pub spawns: Vec<[f32; 3]>,
    /// Uniform scale applied to the GLB and all colliders.
    pub scale: f32,
}

/// Collection of collider boxes for one map.
#[derive(Deserialize, Clone, Debug)]
pub struct ColliderCollection {
    pub colliders: Vec<ColliderBox>,
}

/// Load collider data for a map by name (embedded at compile time).
pub fn load_colliders(name: &str) -> ColliderCollection {
    let json = match name {
        "dust_storm" => include_str!("../assets/maps/dust_storm_colliders.json"),
        "city" => include_str!("../assets/maps/city_colliders.json"),
        _ => panic!("unknown map: {name}"),
    };
    serde_json::from_str(json).expect("failed to parse collider JSON")
}

/// Load shared map data (spawns, scale) for a map by name.
pub fn load_map_data(name: &str) -> MapData {
    let json = match name {
        "dust_storm" => include_str!("../assets/maps/dust_storm_data.json"),
        "city" => include_str!("../assets/maps/city_data.json"),
        _ => panic!("unknown map: {name}"),
    };
    serde_json::from_str(json).expect("failed to parse map data JSON")
}
