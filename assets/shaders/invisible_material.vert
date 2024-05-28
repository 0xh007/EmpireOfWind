#version 450
layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;

layout(location = 0) out vec3 frag_normal;

layout(set = 0, binding = 0) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
};

void main() {
    frag_normal = in_normal;
    gl_Position = proj * view * model * vec4(in_position, 1.0);
}
