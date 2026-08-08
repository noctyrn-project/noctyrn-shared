use serde::Deserialize;

/// A triangle mesh collider from one mesh node.
/// parry3d builds a BVH-accelerated TriMesh from this at runtime.
#[derive(Deserialize, Clone, Debug)]
pub struct TriangleMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<[u32; 3]>,
    /// The material name recorded by the bake tool (from the GLB). The game
    /// maps it to a `MaterialType`; `None`/unknown names become the default
    /// "world" material.
    #[serde(default)]
    pub material: Option<String>,
}

/// Per-map spawn point and objective data, shared by server and client.
#[derive(Deserialize, Clone, Debug)]
pub struct MapData {
    pub spawns: Vec<[f32; 3]>,
    pub scale: f32,
}

/// Collection of triangle meshes for one map.
#[derive(Deserialize, Clone, Debug)]
pub struct ColliderCollection {
    pub colliders: Vec<TriangleMesh>,
}

/// Load collider data for a map by name (embedded at compile time).
pub fn load_colliders(name: &str) -> ColliderCollection {
    let json = match name {
        "dust_storm" => include_str!("../assets/maps/dust_storm_colliders.json"),
        "city" => include_str!("../assets/maps/city_colliders.json"),
        "testing_grounds" => include_str!("../assets/maps/testing_grounds_colliders.json"),
        _ => panic!("unknown map: {name}"),
    };
    serde_json::from_str(json).expect("failed to parse collider JSON")
}

/// Load shared map data (spawns, scale) for a map by name.
pub fn load_map_data(name: &str) -> MapData {
    let json = match name {
        "dust_storm" => include_str!("../assets/maps/dust_storm_data.json"),
        "city" => include_str!("../assets/maps/city_data.json"),
        "testing_grounds" => include_str!("../assets/maps/testing_grounds_data.json"),
        _ => panic!("unknown map: {name}"),
    };
    serde_json::from_str(json).expect("failed to parse map data JSON")
}
