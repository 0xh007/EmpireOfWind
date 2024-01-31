use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

use crate::prelude::*;

const SHIP_LENGTH: i32 = 40;
const SHIP_WIDTH: i32 = 10;
const SHIP_HEIGHT: i32 = 12;

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
            transform: Transform::from_xyz(-4.0, SHIP_HEIGHT as f32 + (0.5 + 2.0) / 2.0, 1.0),
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
    assets: Res<AssetServer>,
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
            material: materials.add(Color::hex("A0522D").unwrap().into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Deck,
        RigidBody::Static,
        Collider::cuboid(SHIP_LENGTH as f32, wall_thickness, SHIP_WIDTH as f32),
        NavMeshAffector,
    ));

    // Top deck of the ship
    // commands.spawn((
    //     Name::new("Top Deck"),
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Box::new(
    //             SHIP_LENGTH as f32,
    //             wall_thickness,
    //             SHIP_WIDTH as f32,
    //         ))),
    //         material: materials.add(Color::hex("A0522D").unwrap().into()),
    //         transform: Transform::from_xyz(0.0, SHIP_HEIGHT as f32, 0.0),
    //         ..default()
    //     },
    //     Deck,
    //     RigidBody::Static,
    //     Collider::cuboid(SHIP_LENGTH as f32, wall_thickness, SHIP_WIDTH as f32),
    //     NavMeshAffector,
    // ));

    // Create port side of hull
    commands.spawn((
        Name::new("Port Hull"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                SHIP_LENGTH as f32,
                SHIP_HEIGHT as f32 - wall_thickness,
                wall_thickness,
            ))),
            material: materials.add(Color::hex("A0522D").unwrap().into()),
            transform: Transform::from_xyz(
                0.0,
                SHIP_HEIGHT as f32 / 2.0,
                -((SHIP_WIDTH as f32 - wall_thickness) / 2.0),
            ),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(
            SHIP_LENGTH as f32,
            SHIP_HEIGHT as f32 - wall_thickness,
            wall_thickness,
        ),
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
            material: materials.add(Color::hex("A0522D").unwrap().into()),
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
            material: materials.add(Color::hex("A0522D").unwrap().into()),
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

    // Create internal decks
    let space_between_decks = (SHIP_HEIGHT as f32 - 2.0 * wall_thickness) / 4.0;
    // let stairs_asset = assets.load("models/export/stairs/stairs_1m.glb#Scene0");
    for i in 1..=4 {
        println!("Deck: {}", i);
        let lower_deck_height = if i == 1 {
            0.0
        } else {
            wall_thickness + (space_between_decks * (i - 1) as f32)
        };
        println!("Lower Deck Height: {}", lower_deck_height);

        let upper_deck_height = if i == 4 {
            SHIP_HEIGHT as f32
        } else {
            wall_thickness + (space_between_decks * i as f32)
        };

        println!("Upper Deck Height: {}", upper_deck_height);
        let height_diff = upper_deck_height - lower_deck_height;
        println!("Height Diff: {}", height_diff);
        let stair_section_height = 1.0;

        // Calculate the number of stair sections needed
        let num_stairs = (height_diff / stair_section_height).ceil() as i32;
        println!("Number of stairs: {}", num_stairs);

        // TODO: Reactivate this once hull shapes are working
        // for j in 0..num_stairs {
        //     let stair_height = lower_deck_height + (j as f32 * stair_section_height);
        //     println!("Stair {} Y Position: {}", j, stair_height);
        //     let stair_x_position = -(j as f32 * stair_section_height);
        //     println!("Stair {} X Position: {}", j, stair_x_position);
        //     // let stair_z_position = 0.0;
        //
        //     commands.spawn((
        //         Name::new(format!("Stair Section {}", j)),
        //         SceneBundle {
        //             scene: stairs_asset.clone(),
        //             transform: Transform::from_xyz(stair_x_position, stair_height, 0.0),
        //             ..default()
        //         },
        //     ));
        // }

        let deck_height = wall_thickness + (space_between_decks * i as f32);

        commands.spawn((
            Name::new(format!("Internal Deck {}", i)),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    SHIP_LENGTH as f32,
                    wall_thickness,
                    SHIP_WIDTH as f32,
                ))),
                material: materials.add(Color::hex("A0522D").unwrap().into()),
                transform: Transform::from_xyz(0.0, deck_height, 0.0),
                ..default()
            },
            Deck,
            RigidBody::Static,
            Collider::cuboid(SHIP_LENGTH as f32, wall_thickness, SHIP_WIDTH as f32),
            NavMeshAffector,
        ));
    }
}
