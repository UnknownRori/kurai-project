#version 330 core

uniform vec2 screen_size;

out vec2 fragPos;

void main() {
    vec2 vertexPos = (gl_Vertex.xy / screen_size) * 2.0 - 1.0;
    gl_Position = vec4(vertexPos, 0.0, 1.0);
    fragPos = gl_Vertex.xy / screen_size;
}
