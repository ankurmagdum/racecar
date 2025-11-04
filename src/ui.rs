use crate::components::{Health, Player, Racer};
use crate::states::GameState;
use bevy::prelude::*;

pub struct UIPlugin;

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct HealthText;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnEnter(GameState::InRace), setup_hud)
            .add_systems(
                Update,
                (
                    main_menu_system.run_if(in_state(GameState::MainMenu)),
                    update_hud.run_if(in_state(GameState::InRace)),
                ),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu_camera)
            .add_systems(OnExit(GameState::InRace), cleanup_hud);
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

#[derive(Component)]
struct HudUI;

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(20.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            HudUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("speed: 0 km/h"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                SpeedText,
            ));

            parent.spawn((
                Text::new("health: 100%"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 1.0, 0.2)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                HealthText,
            ));
        });
}

fn update_hud(
    player_query: Query<(&Racer, &Health), With<Player>>,
    mut speed_query: Query<&mut Text, (With<SpeedText>, Without<HealthText>)>,
    mut health_query: Query<&mut Text, (With<HealthText>, Without<SpeedText>)>,
) {
    if let Ok((racer, health)) = player_query.single() {
        if let Ok(mut text) = speed_query.single_mut() {
            *text = Text::new(format!("speed: {:.0} km/h", racer.speed.abs() * 3.6));
        }

        if let Ok(mut text) = health_query.single_mut() {
            let health_percent = (health.current / health.max * 100.0).round();
            *text = Text::new(format!("health: {:.0}%", health_percent));
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn()
    }
}
