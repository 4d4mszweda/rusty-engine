#version 330 core

layout (location = 0) in vec3 a_pos;
layout (location = 2) in vec2 a_tex;

// instanced model matrix
layout (location = 3) in vec4 i_m0;
layout (location = 4) in vec4 i_m1;
layout (location = 5) in vec4 i_m2;
layout (location = 6) in vec4 i_m3;

uniform mat4 u_view;
uniform mat4 u_proj;

out vec2 v_tex;

void main() {
    mat4 model = mat4(i_m0, i_m1, i_m2, i_m3);
    v_tex = a_tex;

    vec4 worldPos = model * vec4(a_pos, 1.0);
    gl_Position = u_proj * u_view * worldPos;
}
