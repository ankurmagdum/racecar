use crate::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                main_menu_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu_camera);
    }
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct MainMenuCamera;

fn cleanup_menu_camera(mut commands: Commands, query: Query<Entity, With<MainMenuCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainMenuCamera));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("a racing game"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new("press SPACE to start"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
}

fn main_menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InRace);
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
