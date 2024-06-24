use bevy::prelude::*;

/// An event sent for toggling the visibility of the navigation mesh debug drawing.
///
/// This event is used to enable or disable the debug visualization of the navigation mesh
/// within the game. It can be triggered by various systems, typically in response to user input.
///
/// ## Example usage:
///
/// ```rust
/// use bevy::prelude::EventWriter;
/// use empire_of_wind::events::NavMeshDebugToggle;
///
/// fn some_system(mut event_writer: EventWriter<NavMeshDebugToggle>) {
///     // Trigger the event to toggle the nav mesh debug drawing
///     event_writer.send(NavMeshDebugToggle);
/// }
/// ```
#[derive(Event)]
pub struct NavMeshDebugToggle;
