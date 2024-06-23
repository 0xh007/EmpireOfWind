use bevy::prelude::{Capsule3d, Color, Commands, default, Mesh, ResMut, Transform};
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::math::Vec3;
use big_brain::actions::Steps;
use bevy::core::Name;
use bevy_xpbd_3d::components::{LockedAxes, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_xpbd3d::TnuaXpbd3dSensorShape;
use big_brain::prelude::Thinker;
use big_brain::pickers::FirstToScore;
use crate::components::{Eat, Fatigue, FatigueScorer, Food, Hunger, HungerScorer, MoveToNearest, NavigationPath, Npc, Sleep, SleepArea};

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
