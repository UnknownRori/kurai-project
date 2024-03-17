// INFO : Not so efficient
// https://www.shadertoy.com/view/M3jSW1

#version 330 core

varying lowp vec2 uv;           // From vertex shader
varying lowp vec2 fragTexCoord;

uniform float iTime;            // Coming from uniform
uniform vec2 iResolution;       // Coming from uniform not normalized
uniform mat4 Model;             // Default macroquad model stuff
uniform mat4 Projection;        // Default macroquad camera stuff

const float SCALE = 2.;

float hash(vec2 p)  // TODO : replace this by something better
{
    p  = 50.0*fract( p*0.3183099 + vec2(0.71,0.113));
    return -1.0+2.0*fract( p.x*p.y*(p.x+p.y) );
}

float noise( vec2 p )
{
    #if METHOD==0
    ivec2 i = ivec2(floor( p ));
    #else
    vec2 i = floor( p );
    #endif
    vec2 f = fract( p );
	
    // quintic interpolant
    vec2 u = f*f*f*(f*(f*6.0-15.0)+10.0);

    return mix( mix( hash( i + vec2(0.0,0.0) ), 
                     hash( i + vec2(1.0,0.0) ), u.x),
                mix( hash( i + vec2(0.0,1.0) ), 
                     hash( i + vec2(1.0,1.0) ), u.x), u.y);
}

// -----------------------------------------------

vec3 clouds(vec2 uv, vec2 intensity) {
    uv.y += iTime * 0.25;
	
	float f = 0.0;


    uv *= SCALE;
    mat2 m = mat2( 1.6,  1.2, -1.2,  1.6 );
    f  = 0.5000*noise( uv ); uv = m*uv;
    f += 0.2500*noise( uv ); uv = m*uv;
    f += 0.1250*noise( uv ); uv = m*uv;
    f += 0.0625*noise( uv ); uv = m*uv;

    f = 0.2 + 0.5*f;
    return vec3(f);
}

vec4 water(vec2 uv) {
    vec4 texture_color = vec4(0.002156862745098, 0.2627450980392157, 0.5333333333333333, 1.0);
    
    vec4 k = vec4(iTime)*0.8;
	k.xy = uv * 7.0;
    float val1 = length(0.5-fract(k.xyw*=mat3(vec3(-2.0,-1.0,0.0), vec3(3.0,-1.0,1.0), vec3(1.0,-1.0,-1.0))*0.5));
    float val2 = length(0.5-fract(k.xyw*=mat3(vec3(-2.0,-1.0,0.0), vec3(3.0,-1.0,1.0), vec3(1.0,-1.0,-1.0))*0.2));
    float val3 = length(0.5-fract(k.xyw*=mat3(vec3(-2.0,-1.0,0.0), vec3(3.0,-1.0,1.0), vec3(1.0,-1.0,-1.0))*0.5));
    vec4 color = vec4 ( pow(min(min(val1,val2),val3), 7.0) * 3.0)+texture_color;
    
    return color;
}

float snow(vec2 uv,float scale)
{
	float w=smoothstep(1.,0.,-uv.y*(scale/10.));if(w<.1)return 0.;
	uv+=iTime/scale;uv.y+=iTime*2./scale;uv.x+=sin(uv.y+iTime*.5)/scale;
	uv*=scale;vec2 s=floor(uv),f=fract(uv),p;float k=3.,d;
	p=.5+.35*sin(11.*fract(sin((s+p+scale)*mat2(7,3,6,5))*5.))-f;d=length(p);k=min(d,k);
	k=smoothstep(0.,k,sin(f.x+f.y)*0.01);
    	return k*w;
}


void main()
{
    vec2 uvSnow=(fragTexCoord.xy * 200.-iResolution.xy)/min(iResolution.x,iResolution.y); 
    vec4 final;

    final = water(uv);
    final += vec4(clouds(-uv, vec2(0.3, 0.5)), 1.);
    final += vec4(clouds(-uv + 6., vec2(0.3, 0.5)), 1.);
    // final += vec4(clouds(-uv + 12., vec2(0.3, 0.5)), 1.);

    final += vec4(vec3(snow(-uvSnow , 25.)), 0.8);
    final = mix(final, vec4(0.), vec4(0.78));
	
    gl_FragColor = final;
}
