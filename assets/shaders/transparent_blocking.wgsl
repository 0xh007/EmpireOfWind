@group(0) @binding(0) var<uniform> material: StandardMaterialUniform;
@group(1) @binding(0) var<uniform> view: ViewUniform;
@group(1) @binding(1) var<uniform> transform: TransformUniform;
@group(1) @binding(2) var<uniform> mesh: MeshUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let world_position = mesh.model * vec4<f32>(input.position, 1.0);
    output.clip_position = view.projection * view.view * world_position;
    output.world_position = (transform.model * vec4<f32>(input.position, 1.0)).xyz;
    output.world_normal = normalize((transform.model * vec4<f32>(input.normal, 0.0)).xyz);
    return output;
}

@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    if (material.base_color.a < 1.0) {
        discard;
    }
    return vec4<f32>(material.base_color.rgb, 1.0);
}
