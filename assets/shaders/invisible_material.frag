#version 450
layout(location = 0) in vec3 frag_normal;
layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform Material {
    vec4 color;
};

void main() {
    if (color.a < 1.0) {
        discard;
    }
    out_color = color;
}
