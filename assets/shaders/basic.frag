#version 330 core

in vec3 v_normal;
in vec3 v_world_pos;
in vec2 v_tex;

uniform vec3 u_color1;
uniform vec3 u_color2;
uniform int u_is_ground;

// tekstury
uniform sampler2D u_diffuse;
uniform int u_use_texture;     // 1 = używamy tekstury, 0 = gradient
uniform int u_alpha_cutout;    // 1 = używamy alpha discard

out vec4 FragColor;

void main() {
    vec3 N = normalize(v_normal);
    vec3 light_dir = normalize(vec3(0.3, 1.0, 0.5));
    float diff = max(dot(N, light_dir), 0.0);

    float t;
    if (u_is_ground == 1) {
        t = clamp(v_world_pos.x * 0.1 + 0.5, 0.0, 1.0);
    } else {
        t = N.y * 0.5 + 0.5;
    }

    vec3 base_color = mix(u_color1, u_color2, t);

    if (u_use_texture == 1) {
        vec4 texColor = texture(u_diffuse, v_tex);

        if (u_alpha_cutout == 1 && texColor.a < 0.5) {
            discard;
        }

        // mnożenie tekstury przez gradient t
        base_color = texColor.rgb * base_color;
    }

    vec3 final_color = base_color * (0.3 + 0.7 * diff);
    FragColor = vec4(final_color, 1.0);
}
