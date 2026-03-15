use sha2::{Digest, Sha256};

pub fn deterministic_position(seed: &str, radius: f32) -> [f32; 3] {
    let hash = Sha256::digest(seed.as_bytes());
    let angle = to_unit(hash[0], hash[1]) * std::f32::consts::TAU;
    let tilt = (to_unit(hash[2], hash[3]) - 0.5) * 0.8;
    let z = (to_unit(hash[4], hash[5]) - 0.5) * radius * 0.15;

    let x = radius * angle.cos();
    let y = radius * angle.sin() * (1.0 + tilt);

    [x, y, z]
}

pub fn orbit_radius(rank: usize) -> f32 {
    8.0 + (rank as f32 * 2.75)
}

pub fn planet_size(commits: i32) -> f32 {
    let commits = commits.max(1) as f32;
    0.8 + commits.log10() * 1.2
}

fn to_unit(a: u8, b: u8) -> f32 {
    let value = u16::from_be_bytes([a, b]);
    (value as f32) / (u16::MAX as f32)
}
