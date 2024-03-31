#version 100

attribute vec3 position;		// Already normalized so don't normalize this shit

varying lowp vec2 uv;			// For fragment shader
varying lowp vec2 fragTexCoord;

uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform mediump mat4 Model;             // Default macroquad model stuff
uniform mediump mat4 Projection;        // Default macroquad camera stuff

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = position.xy;
}
