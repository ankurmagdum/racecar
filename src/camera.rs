use crate::components::Player;
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
        app.add_systems(OnEnter(GameState::InRace), spawn_camera)
            .add_systems(Update, follow_player.run_if(in_state(GameState::InRace)));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        FollowCamera::default(),
    ));
}

fn follow_player(
    mut camera_query: Query<(&mut Transform, &FollowCamera), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok((mut camera_transform, follow_cam)) = camera_query.single_mut() {
            let target_position = player_transform.translation + follow_cam.offset;

            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, time.delta_secs() * follow_cam.smoothness);

            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}
