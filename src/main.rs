use bevy::prelude::*;
use bevy::{pbr::CascadeShadowConfigBuilder, transform::TransformSystem};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_xpbd_3d::{
    math::*, parry::transformation::vhacd::VHACDParameters, prelude::*, SubstepSchedule, SubstepSet,
};

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 10;

const DECK_OFFSET: f32 = 2.5; // This is the additional amount needed to spawn on the deck

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_RADIUS: f32 = 1.0;
const PLAYER_SPEED: f32 = 500.0;
const MOVEMENT_SMOOTHING_FACTOR: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PanOrbitCameraPlugin)
        .add_event::<MovementAction>()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_ship)
        .add_systems(Startup, spawn_ocean)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, camera_switching)
        .add_systems(
            Update,
            (
                keyboard_input,
                update_grounded,
                apply_deferred,
                apply_gravity,
                movement,
                apply_movement_damping,
            )
                .chain(),
        )
        .add_systems(
            SubstepSchedule,
            kinematic_controller_collisions.in_set(SubstepSet::SolveUserConstraints),
        )
        .add_systems(
            PostUpdate,
            move_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )
        .run();
}

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct NPC;

// #[derive(Component)]
// struct MainCamera;
//
// #[derive(Component)]
// struct DebugCamera;

/// An event sent for a movement input action
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
pub struct ControllerGravity(Vector);

/// The maximum angle a slope can have for a character controller to be able to climb and jump. If
/// the slope is steeper than this angle, the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

/// A bundle that contains the components needed for a basic kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Vector::NEG_Y,
            )
            .with_max_time_of_impact(0.2),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

/// Sends ['MovementAction'] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0);

    if direction != Vector2::ZERO {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }
}

/// Updates the ['Grounded'] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal that isn't too
        // steep
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                rotation.rotate(-hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Responds to ['MovementAction'] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    // Precision is adjusted so that the example works with both the 'f32' and 'f64' features.
    let delta_time = time.delta_seconds();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.z -= direction.x * movement_acceleration.0 * delta_time;
                    linear_velocity.x -= direction.y * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

/// Applies ['ControllerGravity'] to character controllers.
fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&ControllerGravity, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with both the 'f32' and 'f64' features.
    let delta_time = time.delta_seconds();

    for (gravity, mut linear_velocity) in &mut controllers {
        linear_velocity.0 += gravity.0 * delta_time;
    }
}

/// Slows down movement in the XZ plane.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

/// This system performs very basic collision response for kinematic character controllers by
/// pushing them along their contact normals by the current penetration depths.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &RigidBody,
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        With<CharacterController>,
    >,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // If the collision didn't happen during this substep, skip the collision
        if !contacts.during_current_substep {
            continue;
        }

        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first or second entity in
        // the collision
        let is_first: bool;
        let (rb, mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers
        if !rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            // Solve each penetrating contact in the manifold
            for contact in manifold.contacts.iter().filter(|c| c.penetration > 0.0) {
                position.0 += normal * contact.penetration;
            }

            // If the slope isn't too steep to walk on but the character is falling, reset vertical
            // velocity.
            if max_slope_angle.is_some_and(|angle| normal.angle_between(Vector::Y).abs() <= angle.0)
                && linear_velocity.y < 0.0
            {
                linear_velocity.y = linear_velocity.y.max(0.0);
            }
        }
    }
}

fn setup(mut commands: Commands) {
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        cascade_shadow_config,
        ..default()
    });
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let player_start_height = SHIP_HEIGHT as f32 + PLAYER_HEIGHT + DECK_OFFSET; // Position player on top of the ship

    // let player_start_x = SHIP_LENGTH as f32 / 2.0;
    // let player_start_z = SHIP_WIDTH as f32 / 2.0;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.4,
                ..default()
            })),
            material: materials.add(Color::YELLOW.into()),
            transform: Transform::from_xyz(0.0, 15.0, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(1.0, 0.4), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(90.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
    ));
}

fn spawn_ship(mut commands: Commands, assets: ResMut<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: assets.load("models/export/ship/ship.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 9.0, 0.0),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
    ));
}

fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(400.0).into()),
        material: materials.add(Color::hex("618f92").unwrap().into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

// fn spawn_ship(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let cube_size = 1.0; // Size of each cube, adjust as needed
//
//     // Spawn the parent entity with a minimal Transform component
//     commands
//         .spawn((
//             TransformBundle::default(),
//             Ship,
//             InheritedVisibility::default(),
//         ))
//         .with_children(|parent| {
//             for x in 0..SHIP_LENGTH {
//                 for z in 0..SHIP_WIDTH {
//                     for y in 0..SHIP_HEIGHT {
//                         // Check if the current position is on the boundary
//                         if x == 0
//                             || x == SHIP_LENGTH - 1
//                             || z == 0
//                             || z == SHIP_WIDTH - 1
//                             || y == 0
//                             || y == SHIP_HEIGHT - 1
//                         {
//                             parent.spawn((
//                                 PbrBundle {
//                                     mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
//                                     material: materials.add(Color::hex("D18251").unwrap().into()),
//                                     transform: Transform::from_xyz(x as f32, y as f32, z as f32),
//                                     ..default()
//                                 },
//                                 RigidBody::Static,
//                                 // DebugRender::default().with_collider_color(Color::YELLOW),
//                                 Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
//                             ));
//                         }
//                     }
//                 }
//             }
//             // Spawn railing
//             for x in 0..SHIP_LENGTH {
//                 for z in 0..SHIP_WIDTH {
//                     let y = SHIP_HEIGHT; // One layer above the ship
//                                          // Spawn railing on edges
//                     if x == 0 || x == SHIP_LENGTH - 1 || z == 0 || z == SHIP_WIDTH - 1 {
//                         parent.spawn((
//                             PbrBundle {
//                                 mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
//                                 material: materials.add(Color::hex("D18251").unwrap().into()),
//                                 transform: Transform::from_xyz(x as f32, y as f32, z as f32),
//                                 ..default()
//                             },
//                             RigidBody::Static,
//                             Collider::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
//                             // DebugRender::default().with_collider_color(Color::YELLOW),
//                         ));
//                     }
//                 }
//             }
//         });
// }
