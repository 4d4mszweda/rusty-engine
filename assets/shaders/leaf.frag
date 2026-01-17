#version 330 core
in vec2 v_tex;
out vec4 FragColor;

uniform sampler2D u_diffuse;
uniform vec3 u_tint;
uniform float u_alphaThreshold; // np 0.5

void main() {
    vec4 tex = texture(u_diffuse, v_tex);
    if (tex.a < u_alphaThreshold) discard;
    FragColor = vec4(tex.rgb * u_tint, 1.0);
}
