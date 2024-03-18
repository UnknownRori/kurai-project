// NOTE : Not fully working since there is no macro expansion probably

#ifndef UTILS_H
#define UTILS_H

// INFO : Default uniform and attribute for vertex
#ifdef IS_VERTEX

attribute vec3 position;        // Already normalized so don't normalize this shit
attribute vec4 color0;          // Color from texture

varying lowp vec2 uv;           // For fragment shader
varying lowp vec2 fragTexCoord;
// varying lowp vec4 color;     // Converted color for fragment

uniform float iTime;            // Coming from uniform
uniform vec2 iResolution;       // Coming from uniform not normalized
uniform mat4 Model;             // Default macroquad model stuff
uniform mat4 Projection;        // Default macroquad camera stuff

#endif // IS_VERTEX

// INFO : Default uniform and attribute for vertex
#ifdef IS_FRAGMENT
varying lowp vec2 uv;           // From vertex shader

uniform float iTime;            // Coming from uniform
uniform vec2 iResolution;       // Coming from uniform not normalized
uniform mat4 Model;             // Default macroquad model stuff
uniform mat4 Projection;        // Default macroquad camera stuff

#endif // IS_FRAGMENT

#endif // UTILS_H
