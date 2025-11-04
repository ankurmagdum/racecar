use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vec3::ZERO,
            angular: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Racer {
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
}

impl Default for Racer {
    fn default() -> Self {
        Self {
            speed: 0.0,
            max_speed: 30.0,
            acceleration: 10.0,
            turn_speed: 2.0,
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component)]
pub struct Road;

#[allow(unused)]
#[derive(Component)]
pub struct RoadSegment {
    pub z_position: f32,
}
