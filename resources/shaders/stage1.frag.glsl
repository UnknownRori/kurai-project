// INFO : Not so efficient
// https://www.shadertoy.com/view/M3jSW1
#version 100

varying mediump vec2 uv;                // From vertex shader
varying mediump vec2 fragTexCoord;

uniform mediump float iTime;            // Coming from uniform
uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform sampler2D Texture;		// Texture from macroquad draw_texture
uniform sampler2D noise_texture;	// Texture from macroquad draw_texture

// #define F length(.5-fract(k.xyw*=mat3(-2,-1,2, 3,-2,1, 1,2,2)*
#define SPEED_GROUND 0.45
#define SPEED_CLOUD 0.25
#define SCALE 0.8

lowp float noise( in lowp vec2 p )
{
    return texture2D(noise_texture, fract(p)).b;
}

lowp vec3 clouds(in lowp vec2 uv, in lowp vec2 intensity) {
    uv.y += iTime * SPEED_CLOUD;
	
    lowp float f = 0.0;

    uv *= SCALE;
    lowp mat2 m = mat2( 1.6,  1.2, -1.2,  1.6 );
    f  = 0.8000*noise( uv ); uv = m*uv;
    f += 0.4500*noise( uv ); uv = m*uv;
    f += 0.1250*noise( uv ); uv = m*uv;
    f += 0.0625*noise( uv ); uv = m*uv;

    f = 0.2 + 0.5*f;
    return vec3(f);
}

// TODO : Improve this
// lowp vec4 water(in lowp vec2 p) {
//     lowp vec4 k = vec4(0.002156862745098, 0.2627450980392157, 0.5333333333333333, 1.0);
//     
//     k.xy = p*(sin(k=vec4(iTime*.2)).w+2.)*2.;
//     k = pow(min(min(F.5)),F.4))),F.3))), 7.)*25.+vec4(0,.35,.5,1);
//
//     return k;
// }

lowp vec4 water(in lowp vec2 p) {
    return vec4(0.002156862745098, 0.2627450980392157, 0.5333333333333333, 1.0);
}

lowp float snow(in lowp vec2 uv, in lowp float scale)
{
    lowp float w=smoothstep(1.,0.,-uv.y*(scale/10.));if(w<.1)return 0.;
    uv+=iTime/scale;uv.y+=iTime*2./scale;uv.x+=sin(uv.y+iTime*.5)/scale;
    uv*=scale;lowp vec2 s=floor(uv),f=fract(uv),p;lowp float k=3.,d;
    p=.5+.35*sin(11.*fract(sin((s+p+scale)*mat2(7,3,6,5))*5.))-f;d=length(p);k=min(d,k);
    k=smoothstep(0.,k,sin(f.x+f.y)*0.01);
    return k*w;
}

void main()
{
    lowp vec4 final;

    lowp vec2 textureUv = vec2(uv.x, fract(-uv.y  + iTime * SPEED_GROUND));
    final = texture2D(Texture, textureUv) * 0.4;
    final += water(uv);
    // final = mix(final, vec4(0.), vec4(0.58));
    final += vec4(clouds(-uv, vec2(0.3, 0.5)), 1.);
    final += vec4(clouds(-uv + 6., vec2(0.3, 0.5)), 1.);
    final = mix(final, vec4(0.), vec4(0.48));

    lowp vec2 uvSnow=(fragTexCoord.xy * 200.-iResolution.xy)/min(iResolution.x,iResolution.y); 
    final += vec4(vec3(snow(-uvSnow , 25.)), 0.8);
    final = mix(final, vec4(0.), vec4(0.58));
	
    gl_FragColor = final;
}
