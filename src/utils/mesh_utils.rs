use bevy::math::Vec3;
use bevy::prelude::{Assets, Children, Color, Commands, default, Entity, Handle, Mesh, Query, ResMut, StandardMaterial, Transform};
use bevy::render::mesh::VertexAttributeValues;

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

fn visualize_bounds(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    bounds: (Vec3, Vec3),
) {
    let bbox_size = bounds.1 - bounds.0;
    let bbox_position = (bounds.0 + bounds.1) * 0.5;

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(
            bbox_size.x,
            bbox_size.y,
            bbox_size.z,
        )),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
        transform: Transform::from_translation(bbox_position),
        ..default()
    });
}


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
    println!("Calculated Bounds: Min: {:?}, Max: {:?}", min, max);
    (min, max)
}
