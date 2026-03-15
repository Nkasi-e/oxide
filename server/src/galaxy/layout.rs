use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalaxyLayout {
    pub star: Star,
    pub planets: Vec<Planet>,
    pub asteroid_belt: AsteroidBelt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub name: String,
    pub size: f32,
    pub brightness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub username: String,
    pub size: f32,
    pub position: [f32; 3],
    pub commits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsteroidBelt {
    pub count: usize,
}
