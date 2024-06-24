use bevy::prelude::*;

use crate::components::Buoyancy;
use crate::components::voxel_visual::VoxelVisual;
use crate::constants::voxel::VOXEL_SIZE;

fn visualize_voxel_grid(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Buoyancy), Changed<Buoyancy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let voxel_visual_size = VOXEL_SIZE * 0.95; // Adjust size for visual gaps

    for (_entity, transform, buoyancy) in query.iter() {
        for voxel in &buoyancy.voxels {
            if voxel.is_solid {
                // Transform for each voxel based on its position relative to the parent entity
                let voxel_position = transform.translation + voxel.position;

                // Spawn visual representation for each solid voxel
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(voxel_visual_size, voxel_visual_size, voxel_visual_size)),
                        material: materials.add(Color::rgb(0.5, 0.5, 1.0)), // Custom color
                        transform: Transform::from_translation(voxel_position),
                        ..default()
                    })
                    .insert(VoxelVisual {}); // Mark it visually if needed for tracking/deletion
            }
        }
    }
}