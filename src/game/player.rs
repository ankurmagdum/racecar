use crate::components::{Health, Player, Racer, Velocity};
use crate::states::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InRace), spawn_player)
            .add_systems(
                Update,
                (player_input, player_movement).run_if(in_state(GameState::InRace)),
            )
            .add_systems(OnExit(GameState::InRace), cleanup_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.1, 0.1))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
        Racer::default(),
        Velocity::default(),
        Health::default(),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Racer, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut racer) = query.single_mut() {
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            racer.speed =
                (racer.speed + racer.acceleration * time.delta_secs()).min(racer.max_speed);
        } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            racer.speed =
                (racer.speed - racer.acceleration * time.delta_secs()).max(-racer.max_speed * 0.5);
        } else {
            racer.speed *= 0.98;
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Racer, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, racer, mut velocity)) = query.single_mut() {
        let mut turn_input = 0.0;
        
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            turn_input -= 1.0;
        }

        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            turn_input += 1.0;
        }

        velocity.angular = turn_input * racer.turn_speed;
        transform.rotate_y(-velocity.angular * time.delta_secs());

        let forward = transform.back();
        
        velocity.linear = forward * racer.speed;
        transform.translation += velocity.linear * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-5.0, 5.0);
    }
}

fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
