#version 100

varying mediump vec2 uv;                // From vertex shader

uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform sampler2D Texture;		// Texture from macroquad draw_texture
uniform sampler2D _ScreenTexture;       // Texture from macroquad draw_texture

uniform int horizontal;

#define TIME_GRADIENT 25.
#define OFFSET 3.5
#define GRADIENT 0.80
#define DEPTH 4

lowp vec4 blur(sampler2D image, lowp vec2 uv, lowp vec2 resolution, lowp vec2 direction) {
  lowp vec4 color = vec4(0.0);
  lowp vec2 off1 = vec2(1.3333333333333333) * direction;
  color += texture2D(image, uv) * 0.29411764705882354;
  color += texture2D(image, uv + (off1 / resolution)) * 0.35294117647058826;
  color += texture2D(image, uv - (off1 / resolution)) * 0.35294117647058826;
  return color;
}

void main() {
   lowp vec2 tex_offset = vec2(OFFSET);
   lowp vec4 color = vec4(0);

   if (horizontal > 0) {
      for (int i = 0; i < DEPTH; i++) {
	 lowp float gradient = smoothstep(GRADIENT, 0., float(i) / TIME_GRADIENT);
	 color += blur(Texture, uv, iResolution, vec2(tex_offset.x * float(i), 0.)) * gradient;
      }
   } else {
      for (int i = 0; i < DEPTH; i++) {
	 lowp float gradient = smoothstep(GRADIENT, 0., float(i) / TIME_GRADIENT);
	 color += blur(_ScreenTexture, uv, iResolution, vec2(0., tex_offset.y * float(i))) * gradient;
      }
   }

   gl_FragColor = color;
}
