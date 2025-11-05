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
            .add_systems(
                Update,
                handle_quit_input.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct MainMenuUI;

fn setup_main_menu(mut commands: Commands) {
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
                Text::new("racecar"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new("press SPACE to start"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(50.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("press Q to quit"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
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

fn handle_quit_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        info!("Exiting...");
        exit.write(AppExit::Success);
    }
}
