
#version 330 core
layout (location = 0) in vec3 a_pos;

out vec3 v_dir;

uniform mat4 u_view;
uniform mat4 u_proj;

void main() {
    v_dir = a_pos;

    vec4 pos = u_proj * u_view * vec4(a_pos, 1.0);
    gl_Position = pos.xyww; // trick: depth = 1.0
}
