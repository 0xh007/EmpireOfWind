use crate::prelude::*;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add xpbd in here
        app.register_type::<ColliderMarker>()
            .add_systems(Update, read_colliders.run_if(in_state(AppStates::Next)));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColliderMarker;

#[sysfail(log(level = "error"))]
pub fn read_colliders(
    collider_marker: Query<Entity, Added<ColliderMarker>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) -> Result<()> {
    for entity in collider_marker.iter() {
        let mesh = find_mesh(entity, &children, &meshes, &mesh_handles)
            .context("Failed to find mesh for collider")?;
        let collider =
            Collider::trimesh_from_mesh(mesh).context("Failed to create collider from mesh")?;

        commands
            .entity(entity)
            .insert((collider, RigidBody::Static, NavMeshAffector));
    }
    Ok(())
}

fn find_mesh<'a>(
    parent: Entity,
    children_query: &'a Query<&Children>,
    meshes: &'a Assets<Mesh>,
    mesh_handles: &'a Query<&Handle<Mesh>>,
) -> Option<&'a Mesh> {
    if let Ok(children) = children_query.get(parent) {
        for child in children.iter() {
            if let Ok(mesh_handle) = mesh_handles.get(*child) {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    println!("Found a mesh!");
                    return Some(mesh);
                }
            }
        }
    }
    None
}
