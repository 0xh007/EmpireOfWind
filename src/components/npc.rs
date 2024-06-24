use bevy::prelude::*;

/// A marker component indicating that an entity is an NPC (non-player character).
///
/// This component is used to distinguish entities that represent NPCs within the game world.
///
/// # Example
///
/// ```
/// use bevy::prelude::*;
/// use empire_of_wind::components::Npc;
///
/// fn setup(mut commands: Commands) {
///     // Create an entity representing an NPC
///     let npc_entity = commands
///         .spawn((
///             Npc,
///             Transform::default(),
///             // Additional components specific to the NPC entity
///         ))
///         .current_entity()
///         .unwrap();
///
///     // Access the NPC component
///     let npc_component = Commands::current_world().get_component::<Npc>(npc_entity).unwrap();
///     println!("NPC component: {:?}", npc_component);
/// }
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_startup_system(setup)
///         .run();
/// }
/// ```
#[derive(Component)]
pub struct Npc;
