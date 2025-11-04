use bevy::prelude::*;

#[allow(unused, dead_code)]
#[derive(Resource)]
pub struct RaceState {
    pub distance: f32,
    pub position: u32,
    pub lap: u32,
}

impl Default for RaceState {
    fn default() -> Self {
        Self {
            distance: 0.0,
            position: 1,
            lap: 1,
        }
    }
}

#[allow(unused, dead_code)]
#[derive(Resource)]
pub struct GameSettings {
    pub total_laps: u32,
    pub num_opponents: u32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            total_laps: 3,
            num_opponents: 5,
        }
    }
}
