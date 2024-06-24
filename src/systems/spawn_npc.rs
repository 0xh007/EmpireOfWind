use bevy::asset::Assets;
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Capsule3d, Color, Commands, default, Mesh, ResMut, Transform};
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_xpbd3d::TnuaXpbd3dSensorShape;
use bevy_xpbd_3d::components::{LockedAxes, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use big_brain::actions::Steps;
use big_brain::pickers::FirstToScore;
use big_brain::prelude::Thinker;

use crate::components::{Eat, Fatigue, FatigueScorer, Food, Hunger, HungerScorer, MoveToNearest, NavigationPath, Npc, Sleep, SleepArea};

/// Spawns a set of NPCs in the game world.
///
/// This system demonstrates how to integrate multiple crates for advanced behaviors:
///
/// - `big_brain`: Utility AI for defining complex behaviors.
/// - `bevy_xpbd_3d`: Components for 3D physics.
/// - `bevy_tnua`: Movement and navigation.
///
/// Each NPC is configured with basic AI to handle eating and sleeping routines. The AI is managed
/// using the `big_brain` crate, which allows for defining scorers and actions. The NPCs also utilize
/// `bevy_xpbd_3d` for physics and `bevy_tnua` for movement and control.
pub fn spawn_npc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Define the starting point for the NPCs.
    let start_position = Vec3::new(0.0, 8.0, -4.0);
    let spacing = 1.0; // Spacing between each NPC.

    let num_npcs = 8;
    for i in 0..num_npcs {
        let move_and_eat = Steps::build()
            .label("MoveAndEat")
            .step(MoveToNearest::<Food> {
                speed: 1.5,
                _marker: std::marker::PhantomData,
            })
            .step(Eat {
                until: 10.0,
                per_second: 10.0,
            });

        let move_and_sleep = Steps::build()
            .label("MoveAndSleep")
            .step(MoveToNearest::<SleepArea> {
                speed: 1.5,
                _marker: std::marker::PhantomData,
            })
            .step(Sleep {
                until: 10.0,
                per_second: 15.0,
            });

        let position = start_position + Vec3::new(0.0, 0.0, spacing * i as f32);

        commands.spawn((
            Name::new("NPC"),
            PbrBundle {
                mesh: meshes.add(Capsule3d {
                    radius: 0.4,
                    ..default()
                }),
                material: materials.add(Color::YELLOW),
                transform: Transform::from_translation(position),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(0.5, 0.5),
            TnuaControllerBundle::default(),
            TnuaXpbd3dSensorShape(Collider::cylinder(0.0, 0.49)),
            LockedAxes::ROTATION_LOCKED,
            Npc,
            Hunger {
                is_eating: false,
                per_second: 4.0,
                level: 0.0,
            },
            Fatigue {
                is_sleeping: false,
                per_second: 4.0,
                level: 0.0,
            },
            NavigationPath::default(),
            Thinker::build()
                .label("NPC Thinker")
                // Selects the action with the highest score that is above the threshold
                .picker(FirstToScore::new(0.6))
                .when(FatigueScorer, move_and_sleep)
                .when(HungerScorer, move_and_eat),
        ));
    }
}
