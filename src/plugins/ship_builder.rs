use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 10;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Deck;

pub struct ShipBuilderPlugin;

impl Plugin for ShipBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_ship);
    }
}

fn generate_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Thickness of the ship's walls, bottom, and top deck
    let wall_thickness = 0.5;
    let half_height = SHIP_HEIGHT as f32 / 2.0;
    let half_length = SHIP_LENGTH as f32 / 2.0;
    let half_width = SHIP_WIDTH as f32 / 2.0;
    let height = SHIP_HEIGHT as f32 - wall_thickness; // Subtract top and bottom thickness

    // Create the bottom of the ship
    commands.spawn((
        Name::new("Bottom Deck"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                SHIP_LENGTH as f32,
                wall_thickness,
                SHIP_WIDTH as f32,
            ))),
            material: materials.add(Color::hex("D18251").unwrap().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Deck,
        RigidBody::Static,
        Collider::cuboid(SHIP_LENGTH as f32, wall_thickness, SHIP_WIDTH as f32),
        NavMeshAffector,
    ));

    // Top deck of the ship
    commands.spawn((
        Name::new("Top Deck"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                SHIP_LENGTH as f32,
                wall_thickness,
                SHIP_WIDTH as f32,
            ))),
            material: materials.add(Color::hex("A0522D").unwrap().into()),
            transform: Transform::from_xyz(0.0, SHIP_HEIGHT as f32, 0.0),
            ..default()
        },
        Deck,
        RigidBody::Static,
        Collider::cuboid(SHIP_LENGTH as f32, wall_thickness, SHIP_WIDTH as f32),
        NavMeshAffector,
    ));

    // Starboard side of the hull

    // Create port side of hull

    // Create bow
    commands.spawn((
        Name::new("Bow"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                wall_thickness,
                SHIP_HEIGHT as f32 + (wall_thickness),
                SHIP_WIDTH as f32,
            ))),
            material: materials.add(Color::hex("8B4513").unwrap().into()),
            transform: Transform::from_xyz(
                -((SHIP_LENGTH as f32 / 2.0) + (wall_thickness / 2.0)),
                SHIP_HEIGHT as f32 / 2.0,
                0.0,
            ),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(wall_thickness, SHIP_HEIGHT as f32, SHIP_WIDTH as f32),
    ));

    // Create Stern
    commands.spawn((
        Name::new("Stern"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                wall_thickness,
                SHIP_HEIGHT as f32 + (wall_thickness),
                SHIP_WIDTH as f32,
            ))),
            material: materials.add(Color::hex("8B4513").unwrap().into()),
            transform: Transform::from_xyz(
                (SHIP_LENGTH as f32 / 2.0) + (wall_thickness / 2.0),
                SHIP_HEIGHT as f32 / 2.0,
                0.0,
            ),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(wall_thickness, SHIP_HEIGHT as f32, SHIP_WIDTH as f32),
    ));
}
