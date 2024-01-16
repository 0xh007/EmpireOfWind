use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};
use oxidized_navigation::NavMeshAffector;

use crate::prelude::*;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npc);
    }
}

fn spawn_npc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Define the starting point for the NPCs.
    let start_position = Vec3::new(0.0, 15.0, 2.0);
    let spacing = 1.0; // Spacing between each NPC.

    for i in 0..10 {
        let position = start_position + Vec3::new(0.0, 0.0, spacing * i as f32);

        commands.spawn((
            Name::new("NPC"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.4,
                    ..default()
                })),
                material: materials.add(Color::YELLOW.into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            CharacterControllerBundle::new(Collider::capsule(1.0, 0.4), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(90.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
            Npc,
        ));
    }
}
