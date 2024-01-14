use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 10;

#[derive(Component)]
struct Ship;

pub struct ShipBuilderPlugin;

impl Plugin for ShipBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship);
    }
}

fn spawn_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
}

fn generate_deck() {}

fn generate_rails() {}

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
