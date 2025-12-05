#version 330 core

in vec3 v_normal;
in vec3 v_world_pos;

uniform vec3 u_color1;
uniform vec3 u_color2;
uniform int u_is_ground;

out vec4 FragColor;

void main() {
    vec3 N = normalize(v_normal);

    // światło z góry i lekko z boku
    vec3 light_dir = normalize(vec3(0.3, 1.0, 0.5));
    float diff = max(dot(N, light_dir), 0.0);

    // gradient:
    // - dla podłoża: w zależności od pozycji X
    // - dla obiektów: zależność od normalnej (N.y)
    float t;
    if (u_is_ground == 1) {
        t = clamp(v_world_pos.x * 0.1 + 0.5, 0.0, 1.0);
    } else {
        t = N.y * 0.5 + 0.5;
    }

    vec3 base_color = mix(u_color1, u_color2, t);

    vec3 final_color = base_color * (0.3 + 0.7 * diff);

    FragColor = vec4(final_color, 1.0);
}
