use bevy::prelude::*;

pub mod player;
pub mod road;

use player::PlayerPlugin;
use road::RoadPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, RoadPlugin));
    }
}
