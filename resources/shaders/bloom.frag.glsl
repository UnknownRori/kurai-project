#version 100

varying mediump vec2 uv;                // From vertex shader

uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform mediump mat4 Model;             // Default macroquad model stuff
uniform mediump mat4 Projection;        // Default macroquad camera stuff
uniform sampler2D Texture;		// Texture from macroquad draw_texture
uniform sampler2D _ScreenTexture;       // Texture from macroquad draw_texture

uniform int horizontal;
uniform int depth;

#define TIME_GRADIENT 18.
#define GRADIENT 0.40

lowp vec4 blur(sampler2D image, vec2 uv, vec2 resolution, vec2 direction) {
  lowp vec4 color = vec4(0.0);
  lowp vec2 off1 = vec2(1.3846153846) * direction;
  lowp vec2 off2 = vec2(3.2307692308) * direction;
  color += texture2D(image, uv) * 0.2270270270;
  color += texture2D(image, uv + (off1 / resolution)) * 0.3162162162;
  color += texture2D(image, uv - (off1 / resolution)) * 0.3162162162;
  color += texture2D(image, uv + (off2 / resolution)) * 0.0702702703;
  color += texture2D(image, uv - (off2 / resolution)) * 0.0702702703;
  return color;
}

void main() {
   lowp vec2 tex_offset = vec2(1.5);
   lowp vec4 color = vec4(0);

   if (horizontal > 0) {
      for (int i = 0; i < depth; i++) {
	 lowp float gradient = smoothstep(GRADIENT, 0., float(i) / TIME_GRADIENT);
	 color += blur(Texture, uv, iResolution, vec2(tex_offset.x * float(i), 0.)) * gradient;
      }
   } else {
      for (int i = 0; i < depth; i++) {
	 lowp float gradient = smoothstep(GRADIENT, 0., float(i) / TIME_GRADIENT);
	 color += blur(_ScreenTexture, uv, iResolution, vec2(0., tex_offset.y * float(i))) * gradient;
      }
   }

   gl_FragColor = color;
}
