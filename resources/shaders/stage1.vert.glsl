#version 330 core

uniform vec2 screen_size;

out vec2 fragPos;

const float SCALE = 2.;

#define iTime u_time

// 0: integer hash
// 1: float hash (aliasing based) (don't do this unless you live in the year 2013)
#define METHOD 1

// 0: cubic
// 1: quintic
#define INTERPOLANT 1

#if METHOD==0
float hash( in ivec2 p )  // this hash is not production ready, please
{                         // replace this by something better

    // 2D -> 1D
    int n = p.x*3 + p.y*113;

    // 1D hash by Hugo Elias
	n = (n << 13) ^ n;
    n = n * (n * n * 15731 + 789221) + 1376312589;
    return -1.0+2.0*float( n & 0x0fffffff)/float(0x0fffffff);
}
#else
float hash(vec2 p)  // replace this by something better
{
    p  = 50.0*fract( p*0.3183099 + vec2(0.71,0.113));
    return -1.0+2.0*fract( p.x*p.y*(p.x+p.y) );
}
#endif

float noise( in vec2 p )
{
    #if METHOD==0
    ivec2 i = ivec2(floor( p ));
    #else
    vec2 i = floor( p );
    #endif
    vec2 f = fract( p );
	
    #if INTERPOLANT==1
    // quintic interpolant
    vec2 u = f*f*f*(f*(f*6.0-15.0)+10.0);
    #else
    // cubic interpolant
    vec2 u = f*f*(3.0-2.0*f);
    #endif    

    #if METHOD==0
    return mix( mix( hash( i + ivec2(0,0) ), 
                     hash( i + ivec2(1,0) ), u.x),
                mix( hash( i + ivec2(0,1) ), 
                     hash( i + ivec2(1,1) ), u.x), u.y);
    #else
    return mix( mix( hash( i + vec2(0.0,0.0) ), 
                     hash( i + vec2(1.0,0.0) ), u.x),
                mix( hash( i + vec2(0.0,1.0) ), 
                     hash( i + vec2(1.0,1.0) ), u.x), u.y);
    #endif
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


void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec2 p = fragCoord.xy / iResolution.xy;
	vec2 uv = p*vec2(iResolution.x/iResolution.y,1);
    vec4 final;
    final = water(uv);
    final += vec4(clouds(uv, vec2(0.3, 0.5)), 1.);
    final += vec4(clouds(uv + 6., vec2(0.3, 0.5)), 1.);
    final += vec4(clouds(uv + 12., vec2(0.3, 0.5)), 1.);
    final = mix(final, vec4(0.), vec4(0.7));
	
	fragColor = final;
}
