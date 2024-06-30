use bevy::prelude::*;

use crate::buoyancy_physics::Buoyancy;
use crate::buoyancy_physics::constants::VOXEL_SIZE;
use crate::buoyancy_physics::VoxelVisual;

/// Visualizes the voxel grid for debugging purposes.
///
/// This system is used to visualize the voxel grid associated with entities that have
/// a `Buoyancy` component. It draws solid voxels in the game world to help developers
/// understand and debug the voxelization and buoyancy calculations.
///
/// # Parameters
///
/// * `commands`: The Commands resource is used to spawn and configure entities for visualizing voxels.
/// * `query`: A Query to retrieve entities with their `Transform` and `Buoyancy` components that have changed.
/// * `meshes`: A mutable reference to the Assets resource containing Mesh objects.
/// * `materials`: A mutable reference to the Assets resource containing StandardMaterial objects.
///
/// # Behavior
///
/// For each entity with a changed `Buoyancy` component:
///
/// 1. The system iterates over the voxels associated with the `Buoyancy` component.
/// 2. For each solid voxel, it calculates its world position based on the entity's transform.
/// 3. It spawns a visual representation of the solid voxel using PBR (Physically Based Rendering) components,
///    with a slightly reduced size to create visual gaps between voxels.
///
/// This visualization helps developers see the results of the voxelization process and the solid voxels
/// that contribute to the buoyancy calculations.
pub fn visualize_voxel_grid(
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
                        mesh: meshes.add(Cuboid::new(
                            voxel_visual_size,
                            voxel_visual_size,
                            voxel_visual_size,
                        )),
                        material: materials.add(Color::rgb(0.5, 0.5, 1.0)), // Custom color
                        transform: Transform::from_translation(voxel_position),
                        ..default()
                    })
                    .insert(VoxelVisual {}); // Mark it visually if needed for tracking/deletion
            }
        }
    }
}
