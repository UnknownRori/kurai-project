#version 100

varying mediump vec2 uv;                // From vertex shader

uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform mediump mat4 Model;             // Default macroquad model stuff
uniform mediump mat4 Projection;        // Default macroquad camera stuff

// Scene
uniform sampler2D Texture;		// Texture from macroquad draw_texture
uniform sampler2D stage;		// Texture from macroquad draw_texture

uniform sampler2D bloom;
uniform mediump float exposure;

void main() {
    lowp vec3 main_texture = texture2D(Texture, uv).rgb;
    lowp vec3 bloom_color = texture2D(bloom, uv).rgb;
    lowp vec4 stage_texture = texture2D(stage, uv);
    lowp vec3 result; // TODO : I wanted the bloom just make the thing glow, not additive
    lowp float brightness = dot(stage_texture.rgb, vec3(0.2126, 0.7152, 0.0722));
    result = main_texture + mix(vec3(0.), bloom_color, vec3(brightness * 0.3));
    gl_FragColor = vec4(result, 1.0);
}
