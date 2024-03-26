// INFO : Not so efficient
// https://www.shadertoy.com/view/M3jSW1
#version 100

varying lowp vec2 uv;                // From vertex shader
varying lowp vec2 fragTexCoord;

uniform lowp float iTime;            // Coming from uniform
uniform lowp vec2 iResolution;       // Coming from uniform not normalized
uniform lowp mat4 Model;             // Default macroquad model stuff
uniform lowp mat4 Projection;        // Default macroquad camera stuff
uniform sampler2D Texture;		// Texture from macroquad draw_texture

#define SPEED_GROUND 0.45
#define SPEED_CLOUD 0.25
#define SCALE 2.

lowp float hash(in lowp vec2 p)  // TODO : replace this by something better
{
    p  = 50.0*fract( p*0.3183099 + vec2(0.71,0.113));
    return -1.0+2.0*fract( p.x*p.y*(p.x+p.y) );
}

lowp float noise( in lowp vec2 p )
{
    lowp vec2 i = floor( p );
    lowp vec2 f = fract( p );
	
    // quintic interpolant
    lowp vec2 u = f*f*f*(f*(f*6.0-15.0)+10.0);

    return mix( mix( hash( i + vec2(0.0,0.0) ), 
                     hash( i + vec2(1.0,0.0) ), u.x),
                mix( hash( i + vec2(0.0,1.0) ), 
                     hash( i + vec2(1.0,1.0) ), u.x), u.y);
}

// -----------------------------------------------

lowp vec3 clouds(in lowp vec2 uv, in lowp vec2 intensity) {
    uv.y += iTime * SPEED_CLOUD;
	
    lowp float f = 0.0;


    uv *= SCALE;
    lowp mat2 m = mat2( 1.6,  1.2, -1.2,  1.6 );
    f  = 0.5000*noise( uv ); uv = m*uv;
    f += 0.2500*noise( uv ); uv = m*uv;
    f += 0.1250*noise( uv ); uv = m*uv;
    f += 0.0625*noise( uv ); uv = m*uv;

    f = 0.2 + 0.5*f;
    return vec3(f);
}

lowp vec4 water(in lowp vec2 uv) {
    lowp vec4 texture_color = vec4(0.002156862745098, 0.2627450980392157, 0.5333333333333333, 1.0);
    
    lowp vec4 k = vec4(iTime)*0.8;
	k.xy = uv * 7.0;
    lowp float val1 = length(0.5-fract(k.xyw*=mat3(vec3(-2.0,-1.0,0.0), vec3(3.0,-1.0,1.0), vec3(1.0,-1.0,-1.0))*0.5));
    lowp float val2 = length(0.5-fract(k.xyw*=mat3(vec3(-2.0,-1.0,0.0), vec3(3.0,-1.0,1.0), vec3(1.0,-1.0,-1.0))*0.2));
    lowp float val3 = length(0.5-fract(k.xyw*=mat3(vec3(-2.0,-1.0,0.0), vec3(3.0,-1.0,1.0), vec3(1.0,-1.0,-1.0))*0.5));
    lowp vec4 color = vec4 ( pow(min(min(val1,val2),val3), 7.0) * 3.0)+texture_color;
    
    return color;
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
    lowp vec2 uvSnow=(fragTexCoord.xy * 200.-iResolution.xy)/min(iResolution.x,iResolution.y); 
    lowp vec4 final;

    lowp vec2 textureUv = vec2(uv.x, fract(-uv.y  + iTime * SPEED_GROUND));
    final = texture2D(Texture, textureUv);
    final += water(uv);
    final += vec4(clouds(-uv, vec2(0.3, 0.5)), 1.);
    final += vec4(clouds(-uv + 6., vec2(0.3, 0.5)), 1.);
    // final += vec4(clouds(-uv + 12., vec2(0.3, 0.5)), 1.); // Removed because cause slow draw

    final += vec4(vec3(snow(-uvSnow , 25.)), 0.8);
    final = mix(final, vec4(0.), vec4(0.78));
	
    gl_FragColor = final;
}
