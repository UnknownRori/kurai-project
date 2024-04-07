#version 100

varying mediump vec2 uv;                // From vertex shader

uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform sampler2D Texture;		// Texture from macroquad draw_texture

#define BRIGHTNESS_THRESHOLD 0.5

void main() {
    lowp vec4 color = texture2D(Texture, uv);

    lowp float brightness = dot(color.rgb, vec3(0.2126, 0.7152, 0.0722));
    if (brightness > BRIGHTNESS_THRESHOLD) {
	gl_FragColor = color;
    } else {
	gl_FragColor = vec4(0.);
    }
}
