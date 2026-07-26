use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MapId(pub String);

impl MapId {
    pub fn new(s: &str) -> Self {
        MapId(s.to_string())
    }
}

impl std::fmt::Display for MapId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
