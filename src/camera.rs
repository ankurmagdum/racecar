use crate::components::Car;
use crate::states::GameState;
use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vec3,
    pub smoothness: f32,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 5.0, -10.0),
            smoothness: 5.0,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu_camera)
            .add_systems(OnEnter(GameState::InRace), spawn_hud_camera)
            .add_systems(Update, follow_player.run_if(in_state(GameState::InRace)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu_camera)
            .add_systems(OnExit(GameState::InRace), cleanup_hud_camera);
    }
}

#[derive(Component)]
struct MainMenuCamera;

#[derive(Component)]
struct HudCamera;

fn spawn_main_menu_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainMenuCamera));
}

fn spawn_hud_camera(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 400.0,
        ..default()
    });

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        FollowCamera::default(),
        HudCamera,
    ));
}

fn follow_player(
    mut camera_query: Query<(&mut Transform, &FollowCamera), Without<Car>>,
    player_query: Query<&Transform, With<Car>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok((mut camera_transform, follow_cam)) = camera_query.single_mut() {
            let rotation = player_transform.rotation.clone();
            let rotation = rotation.mul_quat(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
            let offset = rotation.mul_vec3(follow_cam.offset.clone());
            let target_position = player_transform.translation + offset;

            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, time.delta_secs() * follow_cam.smoothness);

            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

fn cleanup_main_menu_camera(mut commands: Commands, query: Query<Entity, With<MainMenuCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn cleanup_hud_camera(mut commands: Commands, query: Query<Entity, With<HudCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
