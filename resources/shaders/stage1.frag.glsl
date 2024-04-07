// INFO : Not so efficient
// https://www.shadertoy.com/view/M3jSW1
#version 100

varying mediump vec2 uv;                // From vertex shader
varying mediump vec2 fragTexCoord;

uniform mediump float iTime;            // Coming from uniform
uniform mediump vec2 iResolution;       // Coming from uniform not normalized
uniform sampler2D Texture;		// Texture from macroquad draw_texture
uniform sampler2D noise_texture;	// Texture from macroquad draw_texture
uniform sampler2D entities_buffer;	// Texture from macroquad draw_texture

#define SPEED_GROUND 0.45
#define SPEED_CLOUD 0.25
#define SCALE 0.8

// How time affect the distortion
#define TIME_MULTIPLIER 0.15
// How much strength of the distortion
#define DISTORTION_STRENGTH 0.15

lowp float noise( in lowp vec2 p )
{
    return texture2D(noise_texture, fract(p)).b;
}

lowp vec4 clouds(in lowp vec2 uv, in lowp vec2 intensity) {
    uv.y += iTime * SPEED_CLOUD;
	
    lowp float f = 0.0;

    uv *= SCALE;
    lowp mat2 m = mat2( 1.6,  1.2, -1.2,  1.6 );
    f  = 0.8000*noise( uv ); uv = m*uv;
    f += 0.4500*noise( uv ); uv = m*uv;
    f += 0.1250*noise( uv ); uv = m*uv;
    f += 0.0625*noise( uv ); uv = m*uv;

    f = 0.2 + 0.5*f;
    return vec4(vec3(f), 1.);
}

lowp float generateDistortion(in lowp float iTime, in lowp vec2 uv)
{
    return noise(uv + iTime * TIME_MULTIPLIER);
}

lowp vec4 water(in lowp vec2 uv) {
    lowp float distortion = generateDistortion(iTime, uv);
    
    // Apply the distortion to uv
    lowp vec2 distortedUv = fragTexCoord;
    distortedUv.y -= distortion * DISTORTION_STRENGTH;
    
    
    // Get base texture based on distorted uv
    lowp vec4 color = texture2D(Texture, distortedUv);
    // TODO : Make sure apply distortion, currently not working properly and causing weird coloring issue
    color += texture2D(entities_buffer, vec2(fragTexCoord.x, fragTexCoord.y - 0.08 - distortedUv.y * 0.1)) * 0.2;
    lowp vec4 topColor = smoothstep(0.7, 0.8, distortion) * vec4(1., 1., 1., 1.);
    
    // Blueish color
    lowp vec4 waterColor = vec4(0.002156862745098, 0.2627450980392157, 0.5333333333333333, 1.0);
    color += waterColor + topColor;
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
    lowp vec4 final;

    lowp vec2 textureUv = vec2(uv.x, fract(-uv.y  + iTime * SPEED_GROUND));
    final += water(textureUv);
    lowp vec4 cloud;
    cloud += clouds(-uv, vec2(0.3, 0.5));
    cloud += clouds(-uv + 6., vec2(0.3, 0.5));

    final += cloud;
    final = mix(final, vec4(0.), vec4(0.58));

    lowp vec2 uvSnow=(fragTexCoord.xy * 200.-iResolution.xy)/min(iResolution.x,iResolution.y); 
    final += vec4(vec3(snow(-uvSnow , 25.)), 0.8);
    final = mix(final, vec4(0.), vec4(0.48));
	
    gl_FragColor = final;
}
