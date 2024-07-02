use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

/// Finds the mesh handle for a given parent entity by traversing its children.
///
/// This function checks if any of the children of the specified parent entity
/// have a `Mesh` component. If a mesh is found, its handle is returned.
///
/// # Parameters
/// - `parent`: The parent entity whose children will be searched for a mesh handle.
/// - `children_query`: Query to fetch the children of entities.
/// - `mesh_handles`: Query to fetch mesh handles from entities.
///
/// # Returns
/// An `Option<Handle<Mesh>>` which is:
/// - `Some(Handle<Mesh>)` if a mesh handle is found among the children.
/// - `None` if no mesh handle is found.
pub fn find_mesh(
    parent: Entity,
    children_query: &Query<&Children>,
    mesh_handles: &Query<&Handle<Mesh>>,
) -> Option<Handle<Mesh>> {
    if let Ok(children) = children_query.get(parent) {
        for child in children.iter() {
            if let Ok(mesh_handle) = mesh_handles.get(*child) {
                return Some(mesh_handle.clone());
            }
        }
    }
    None
}

/// Visualizes the bounding box of a mesh in the game world.
///
/// This function creates a 3D cuboid to represent the bounding box of a mesh,
/// which is useful for debugging purposes, especially in understanding the
/// voxelization process. The bounding box is visualized using a `PbrBundle`
/// with a red color.
///
/// # Parameters
/// - `commands`: The `Commands` resource to spawn and configure entities.
/// - `meshes`: Mutable reference to the `Assets` resource containing `Mesh` objects.
/// - `materials`: Mutable reference to the `Assets` resource containing `StandardMaterial` objects.
/// - `bounds`: A tuple containing the minimum and maximum coordinates (`Vec3`) of the bounding box.
pub fn visualize_bounds(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    bounds: (Vec3, Vec3),
) {
    let bbox_size = bounds.1 - bounds.0;
    let bbox_position = (bounds.0 + bounds.1) * 0.5;

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(bbox_size.x, bbox_size.y, bbox_size.z)),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
        transform: Transform::from_translation(bbox_position),
        ..default()
    });
}

/// Calculates the axis-aligned bounding box (AABB) of a mesh.
///
/// This function computes the minimum and maximum coordinates of a mesh, which define
/// its axis-aligned bounding box. The bounds are calculated based on the positions of
/// the vertices in the mesh.
///
/// # Parameters
/// - `mesh`: Reference to the `Mesh` object whose bounds are to be calculated.
///
/// # Returns
/// A tuple containing two `Vec3` values:
/// - The first value is the minimum coordinate of the bounding box.
/// - The second value is the maximum coordinate of the bounding box.
///
/// # Panics
/// This function will panic if the mesh does not contain position attributes.
pub fn calculate_mesh_bounds(mesh: &Mesh) -> (Vec3, Vec3) {
    let positions = if let Some(VertexAttributeValues::Float32x3(pos)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        pos
    } else {
        panic!("Mesh does not contain position attribute.");
    };

    // Initialize min and max with the first vertex to ensure correctness.
    let mut min = Vec3::new(positions[0][0], positions[0][1], positions[0][2]);
    let mut max = min;

    for &vertex in positions.iter() {
        min = min.min(Vec3::from(vertex));
        max = max.max(Vec3::from(vertex));
    }

    (min, max)
}
