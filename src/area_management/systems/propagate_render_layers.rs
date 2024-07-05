use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// System to propagate `RenderLayers` from parent entities to their child entities.
///
/// This system ensures that child entities inherit the render layers of their parent entities,
/// which is essential for correctly rendering hierarchical scenes where child entities should
/// follow the rendering settings of their parents.
///
/// # Parameters
/// - `commands`: Commands for modifying entities and their components.
/// - `parent_query`: Query to retrieve parent entities with their `RenderLayers` component and children.
/// - `child_query`: Query to retrieve child entities that have a `Handle<Mesh>` and `Handle<StandardMaterial>`,
///                  which are the components indicating the entity is a renderable child.
///
/// # Details
/// For each parent entity with `RenderLayers`, the system iterates over its children and
/// checks if the child entity has a `Handle<Mesh>` and `Handle<StandardMaterial>`. If so,
/// it propagates the `RenderLayers` from the parent to the child entity.
pub fn propagate_render_layers(
    mut commands: Commands,
    parent_query: Query<(&RenderLayers, &Children)>,
    child_query: Query<(Entity, &Handle<Mesh>, &Handle<StandardMaterial>)>,
) {
    for (render_layers, children) in parent_query.iter() {
        for &child in children.iter() {
            if let Ok((child_entity, _, _)) = child_query.get(child) {
                commands.entity(child_entity).insert(*render_layers);
            }
        }
    }
}
