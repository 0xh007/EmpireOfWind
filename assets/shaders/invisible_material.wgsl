@group(0) @binding(0) var<uniform> material: InvisibleMaterialUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vertex_main(@location(0) position: vec3<f32>, @location(1) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.world_position = position;
    output.world_normal = normal;
    output.clip_position = vec4<f32>(position, 1.0);
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // If alpha < 1.0, make the fragment fully transparent but still interacting with light
    if (material.color.a < 1.0) {
        discard;
    }
    return material.color;
}
