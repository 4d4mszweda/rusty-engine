#version 330 core

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_tex;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

out vec3 v_normal;
out vec3 v_world_pos;
out vec2 v_tex;

void main() {
    vec4 world_pos = u_model * vec4(a_pos, 1.0);

    v_world_pos = world_pos.xyz;
    v_normal = mat3(u_model) * a_normal;
    v_tex = a_tex;

    gl_Position = u_proj * u_view * world_pos;
}
