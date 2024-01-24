#version 330 core

in vec2 fragPos;

out vec4 color;

void main() {
    // Light position in normalized screen coordinates
    vec2 lightPos = vec2(0.0, 1.0);

    // Calculate distance from fragment to light
    float distanceToLight = distance(fragPos, lightPos);

    // Define shadow intensity based on distance (you can adjust these values)
    float shadowIntensity = smoothstep(0.2, 0.3, distanceToLight);

    // Output the shadow color (you can adjust color and alpha)
    color = vec4(0.0, 0.0, 0.0, 1.0 - shadowIntensity);
}
