use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_xpbd_3d::prelude::{Collisions, Gravity, Physics, Sensor, SpatialQueryPipeline};
use empire_of_wind::components::ship::Ship;

use empire_of_wind::prelude::*;

#[test]
fn test_read_area_markers() {
    // Create a new app
    let mut app = App::new();

    // Add necessary plugins excluding window and event loop plugins
    app.add_plugins(MinimalPlugins); // Use minimal plugins for the test environment
    app.add_plugins(PhysicsPlugin);

    // Add the necessary resources manually
    app.insert_resource(Time::<Physics>::default());
    app.insert_resource(Gravity::default());
    app.insert_resource(SpatialQueryPipeline::default());
    app.insert_resource(Collisions::default());

    // Setup test resources and entities
    let ship_entity = app.world.spawn(Ship).id();

    // Add a mock mesh to the assets
    app.world.insert_resource(Assets::<Mesh>::default());
    let mut meshes = app.world.resource_mut::<Assets<Mesh>>();
    meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    // Spawn an entity with an AreaEnterMarker and a Transform
    let area_enter_marker_entity = app
        .world
        .spawn((
            AreaEnterMarker,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();

    // Spawn an entity with an AreaExitMarker and a Transform
    let area_exit_marker_entity = app
        .world
        .spawn((
            AreaExitMarker,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();

    // Mock the parent-child relationship for the markers
    app.world
        .entity_mut(ship_entity)
        .push_children(&[area_enter_marker_entity, area_exit_marker_entity]);

    // Add the necessary systems
    app.add_systems(Update, read_area_markers);

    // Manually set the delta time for the Time resource
    let mut time = app.world.resource_mut::<Time<Physics>>();
    time.advance_by(Duration::from_secs_f64(1.0 / 60.0));

    // Run the app to trigger the system
    app.update();

    // Check the resulting changes for AreaEnterMarker
    // assert!(app.world.get::<Collider>(area_enter_marker_entity).is_some());
    assert!(app.world.get::<Sensor>(area_enter_marker_entity).is_some());
    assert!(app
        .world
        .get::<Visibility>(area_enter_marker_entity)
        .is_some());

    // Check the resulting changes for AreaExitMarker
    // assert!(app.world.get::<Collider>(area_exit_marker_entity).is_some());
    assert!(app.world.get::<Sensor>(area_exit_marker_entity).is_some());
    assert!(app
        .world
        .get::<Visibility>(area_exit_marker_entity)
        .is_some());
}
