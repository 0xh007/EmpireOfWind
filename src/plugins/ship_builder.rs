use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

use crate::prelude::*;

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
        app.add_systems(Startup, generate_ship)
            .add_systems(Startup, place_obstacle)
            .add_systems(Startup, place_furniture)
            .add_systems(Startup, place_food);
    }
}

fn place_obstacle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Obstacle"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 2.0, 6.0))),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_xyz(-4.0, SHIP_HEIGHT as f32 + 0.5, 1.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.0, 2.0, 6.0),
        NavMeshAffector,
    ));
}

fn place_furniture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: Move this up
    let wall_thickness = 0.5;
    // Create a bed
    commands.spawn((
        Name::new("Bed"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0, 1.0, 1.0))),
            material: materials.add(Color::MAROON.into()),
            transform: Transform::from_xyz(7.0, SHIP_HEIGHT as f32 + wall_thickness, 0.0),
            ..default()
        },
        SleepArea,
    ));
}

fn place_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: Move this up
    let wall_thickness = 0.5;

    commands.spawn((
        Name::new("Food"),
        Food,
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 20,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(10.0, SHIP_HEIGHT as f32 + wall_thickness, 2.0),
            ..default()
        },
    ));
}

fn generate_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Thickness of the ship's walls, bottom, and top deck
    // TODO: Move this up
    let wall_thickness = 0.5;

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

    // Create side of hull
    commands.spawn((
        Name::new("Port Wall"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                SHIP_LENGTH as f32,
                SHIP_HEIGHT as f32 + wall_thickness,
                wall_thickness,
            ))),
            material: materials.add(Color::hex("8B4513").unwrap().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(0.0, 0.0, 0.0),
    ));

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
