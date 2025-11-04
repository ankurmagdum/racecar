use bevy::prelude::*;

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InRace,
    Paused,
    GameOver,
}
