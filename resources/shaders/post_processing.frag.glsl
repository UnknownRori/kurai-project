#version 100

varying mediump vec2 uv;                // From vertex shader

uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform mediump mat4 Model;             // Default macroquad model stuff
uniform mediump mat4 Projection;        // Default macroquad camera stuff

// Scene
uniform sampler2D Texture;		// Texture from macroquad draw_texture

uniform sampler2D bloom;
uniform mediump float exposure;

void main() {
    lowp vec3 main_texture = texture2D(Texture, uv).rgb;      
    lowp vec3 bloom_color = texture2D(bloom, uv).rgb;
    lowp vec3 result = main_texture + bloom_color * 0.5; // additive blending
    gl_FragColor = vec4(result, 1.0);
}
