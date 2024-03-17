#version 330

attribute vec3 position;        // Already normalized so don't normalize this shit
attribute vec4 color0;          // Color from texture

varying lowp vec2 uv;           // For fragment shader
varying lowp vec2 fragTexCoord;
// varying lowp vec4 color;     // Converted color for fragment

uniform float iTime;            // Coming from uniform
uniform vec2 iResolution;       // Coming from uniform not normalized
uniform mat4 Model;             // Default macroquad model stuff
uniform mat4 Projection;        // Default macroquad camera stuff


void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    fragTexCoord = position.xy;
    uv = position.xy*vec2(iResolution.x/iResolution.y,1);
}
