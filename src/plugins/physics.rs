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
        app.register_type::<AreaMarker>()
            .register_type::<AreaName>()
            .register_type::<ColliderMarker>()
            .register_type::<NavMeshMarker>()
            .add_systems(Update, read_area_markers.run_if(in_state(AppStates::Next)))
            .add_systems(Update, read_colliders.run_if(in_state(AppStates::Next)));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaMarker;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct AreaName(String);

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColliderMarker;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct NavMeshMarker;

// TODO: This can probably be combined with read_colliders()
#[sysfail(log(level = "error"))]
pub fn read_area_markers(
    area_marker_query: Query<(Entity, &AreaMarker), Added<AreaMarker>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) -> Result<()> {
    for (entity, _area_marker) in area_marker_query.iter() {
        let mesh = find_mesh(entity, &children, &meshes, &mesh_handles)
            .context("Failed to find mesh for area collider")?;
        let collider = Collider::trimesh_from_mesh(mesh)
            .context("Failed to create area collider from mesh")?;

        println!("Inserting Sensor");
        commands
            .entity(entity)
            .insert((collider, RigidBody::Static, Sensor, Visibility::Hidden));
    }
    Ok(())
}

#[sysfail(log(level = "error"))]
pub fn read_colliders(
    collider_marker_query: Query<(Entity, Option<&NavMeshMarker>), Added<ColliderMarker>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) -> Result<()> {
    for (entity, nav_mesh_marker_opt) in collider_marker_query.iter() {
        let mesh = find_mesh(entity, &children, &meshes, &mesh_handles)
            .context("Failed to find mesh for collider")?;
        let collider =
            Collider::trimesh_from_mesh(mesh).context("Failed to create collider from mesh")?;

        // Insert the common components, including making the collider invisible
        commands
            .entity(entity)
            .insert((collider, RigidBody::Static, Visibility::Hidden));

        // If the NavMeshMarker is present, also add NavMeshAffector in a separate step
        if nav_mesh_marker_opt.is_some() {
            commands.entity(entity).insert(NavMeshAffector);
        }
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
                    return Some(mesh);
                }
            }
        }
    }
    None
}
