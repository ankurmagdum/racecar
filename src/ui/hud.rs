use crate::components::{Health, Car, Racer};
use crate::GameState;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InRace), setup_hud)
            .add_systems(Update, update_hud.run_if(in_state(GameState::InRace)))
            .add_systems(Update, hud_system.run_if(in_state(GameState::InRace)))
            .add_systems(OnExit(GameState::InRace), cleanup_hud);
    }
}

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct HealthText;

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

            /*
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
            */

            parent.spawn((
                Text::new("press Q to quit race"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn hud_system(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::MainMenu);
    }
}

fn update_hud(
    player_query: Query<(&Racer, &Health), With<Car>>,
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
