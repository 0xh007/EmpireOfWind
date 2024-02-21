use bevy::prelude::*;
use bevy_water::*;

const WATER_HEIGHT: f32 = 2.0;

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaterSettings {
            height: WATER_HEIGHT,
            ..default()
        })
        .add_plugins(WaterPlugin);
        // .add_systems(Startup, setup);
        // app.add_systems(Startup, spawn_ocean);
    }
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//
// }
//
// fn spawn_ocean(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         Name::new("Ocean"),
//         PbrBundle {
//             mesh: meshes.add(shape::Circle::new(400.0).into()),
//             material: materials.add(Color::hex("618f92").unwrap().into()),
//             transform: Transform::from_rotation(Quat::from_rotation_x(
//                 -std::f32::consts::FRAC_PI_2,
//             )),
//             ..default()
//         },
//     ));
// }
