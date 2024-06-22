fn propagate_render_layers(
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