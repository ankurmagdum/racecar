use crate::components::{Car, Health, Racer, Velocity};
use crate::states::GameState;
use bevy::prelude::*;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_car)
            .add_systems(
                Update,
                (car_input, car_movement).run_if(in_state(GameState::InRace)),
            )
            .add_systems(OnExit(GameState::InRace), cleanup_car);
    }
}

fn spawn_car(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("tiara/scene.gltf"))),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
        Car,
        Racer::default(),
        Velocity::default(),
        Health::default(),
    ));
}

fn car_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Racer, With<Car>>,
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

fn car_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Racer, &mut Velocity), With<Car>>,
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

        let forward =
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2).mul_vec3(transform.back().as_vec3());

        velocity.linear = forward * racer.speed;
        transform.translation += velocity.linear * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-5.0, 5.0);
    }
}

fn cleanup_car(mut commands: Commands, query: Query<Entity, With<Car>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
