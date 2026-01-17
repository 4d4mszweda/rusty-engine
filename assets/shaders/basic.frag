#version 330 core

in vec3 v_normal;
in vec3 v_world_pos;
in vec2 v_tex;

uniform samplerCube u_skybox;
uniform int u_envMode;
uniform float u_envMix;
uniform float u_refractEta;
uniform int u_envEnabled; 

uniform vec3 u_color;
uniform int u_is_ground;

uniform sampler2D u_diffuse;
uniform int u_use_texture;     // 1 = tekstura, 0 = gradient
uniform int u_alpha_cutout;    // 1 = alpha discard


struct LightParam {
    vec3 Ambient;
    vec3 Diffuse;
    vec3 Specular;
    vec3 Attenuation; // point only
    vec3 Position;    // point only
    vec3 Direction;   // directional only (world space)
};

uniform int u_lightType; 

struct MaterialParam {
    vec3 Ambient;
    vec3 Diffuse;
    vec3 Specular;
    float Shininess;  
};

uniform LightParam u_light;
uniform MaterialParam u_material;
uniform vec3 u_cameraPos;

uniform int u_useLighting; 
uniform int u_specModel;  

out vec4 FragColor;

float attenuation(vec3 att, float d) {
    return 1.0 / (att.x + att.y * d + att.z * d * d);
}

void main() {
    vec3 N = normalize(v_normal);

    // --- Twoja logika koloru bazowego (gradient/tekstura) ---
    float t;
    if (u_is_ground == 1) {
        t = clamp(v_world_pos.x * 0.1 + 0.5, 0.0, 1.0);
    } else {
        t = N.y * 0.5 + 0.5;
    }

    vec3 base_color = u_color;

    if (u_use_texture == 1) {
        vec4 texColor = texture(u_diffuse, v_tex);

        if (u_alpha_cutout == 1 && texColor.a < 0.5) {
            discard;
        }

        base_color = texColor.rgb * base_color;
    }

    // Jeśli wyłączone oświetlenie – pokazuj “czysty” kolor/teksturę
    if (u_useLighting == 0) {
        FragColor = vec4(base_color, 1.0);
        return;
    }

	vec3 L;
	float latt = 1.0;

	if (u_lightType == 0) {
		// POINT
		vec3 Lvec = u_light.Position - v_world_pos;
		float dist = length(Lvec);
		L = normalize(Lvec);
		latt = attenuation(u_light.Attenuation, dist);
	} else {
		// DIRECTIONAL (promienie równoległe, bez zaniku)
		L = normalize(-u_light.Direction);
		latt = 1.0;
	}

    // wektor do kamery
    vec3 E = normalize(u_cameraPos - v_world_pos);

    // Lambert
    float diff = max(dot(N, L), 0.0);

    // Specular (Phong / Blinn-Phong)
    float spec = 0.0;
    if (diff > 0.0 && u_material.Shininess > 0.0) {
        if (u_specModel == 0) {
            // Phong: R + dot(R, L)^n
            vec3 R = reflect(-E, N);
            spec = pow(max(dot(R, L), 0.0), u_material.Shininess);
        } else {
            // Blinn-Phong: H = normalize(L + E), dot(N,H)^n
            vec3 H = normalize(L + E);
            spec = pow(max(dot(N, H), 0.0), u_material.Shininess);
        }
    }

    // Współczynnik światła (ambient + atten*(diffuse + spec))
    vec3 ambientPart  = u_light.Ambient  * u_material.Ambient;
    vec3 diffusePart  = diff * u_light.Diffuse  * u_material.Diffuse;
    vec3 specularPart = spec * u_light.Specular * u_material.Specular;

    vec3 lightCoef = ambientPart + latt * (diffusePart + specularPart);

    // Final: oświetlenie moduluje kolor bazowy
    vec3 final_color = lightCoef * base_color;

	if (u_envEnabled == 1 && u_envMode != 0) {
		vec3 V = normalize(u_cameraPos - v_world_pos); 
		vec3 I = -V;                                   

		vec3 dir;
		if (u_envMode == 1) {
			dir = reflect(I, N);
		} else {
			dir = refract(I, N, u_refractEta);
		}

		vec3 env = texture(u_skybox, dir).rgb;

		final_color = mix(final_color, env, clamp(u_envMix, 0.0, 1.0));
	}

    FragColor = vec4(final_color, 1.0);
}
