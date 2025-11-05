use bevy::prelude::*;

pub mod car;
pub mod road;

use car::CarPlugin;
use road::RoadPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarPlugin, RoadPlugin));
    }
}
