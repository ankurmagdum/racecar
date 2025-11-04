use crate::components::{Road, RoadSegment};
use crate::states::GameState;
use bevy::prelude::*;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InRace), spawn_road);
    }
}

fn spawn_road(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let road_width = 12.0;
    let segment_length = 20.0;
    let num_segments = 20;

    for i in 0..num_segments {
        let z_pos = i as f32 * segment_length;

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(road_width, segment_length))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.35))),
            Transform::from_xyz(0.0, 0.0, z_pos),
            //.with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            Road,
            RoadSegment { z_position: z_pos },
        ));
    }

    for i in 0..num_segments * 4 {
        let z_pos = i as f32 * (segment_length / 4.0);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.05, 2.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 0.02, z_pos),
        ));
    }
}
