#version 100

varying mediump vec2 uv;                // From vertex shader
varying mediump vec2 fragTexCoord;

uniform mediump float iTime;            // Coming from uniform
uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform sampler2D Texture;		// Texture from macroquad draw_texture

void main() {
   lowp vec4 color = texture2D(Texture, uv);
   gl_FragColor = color;
}
