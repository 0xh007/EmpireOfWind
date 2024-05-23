@group(1) @binding(0) var<uniform> time: f32;
@group(1) @binding(1) var<uniform> color: vec4<f32>;

@fragment
fn fragment_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}
