#version 450

#define iResolution vec3(2048., 1440., 1.)

layout(location = 0) in vec3 vPos;

layout(location = 0) out vec4 outColor;

layout(set=0, binding=0)
uniform Uniforms {
    float iTime;
};

#define ANIMATE
//#define SINGLE_INDEX 3
//#define INPUT_NOISERANGE
//#define BRUTEFORCE_GAUSSIAN

// ====

// This set suits the coords of of 0-1.0 ranges..
#define MOD3 vec3(443.8975,397.2973, 491.1871)
//#define MOD4 vec4(443.8975,397.2973, 491.1871, 470.7827)

//----------------------------------------------------------------------------------------
//  1 out, 1 in...
float hash11(float p)
{
	vec3 p3  = fract(vec3(p) * MOD3);
    p3 += dot(p3, p3.yzx + 19.19);
    return fract(p3.x * p3.y * p3.z);
}
//----------------------------------------------------------------------------------------
//  1 out, 2 in...
float hash12(vec2 p)
{
	vec3 p3  = fract(vec3(p.xyx) * MOD3);
    p3 += dot(p3, p3.yzx + 19.19);
    return fract(p3.x * p3.z * p3.y);
}

float remap( float t, float a, float b ) {
	return clamp( (t - a) / (b - a), 0.0, 1.0 );
}

const float NUM_LEVELS_F = 64.0;

//note: from https://www.shadertoy.com/view/MlVSzw
float inv_error_function(float x)
{
	const float ALPHA = 0.14;
	const float INV_ALPHA = 1.0 / ALPHA;
    const float PI = 3.1415926535;
	const float K = 2.0 / (PI * ALPHA);

    float y = log(1.0 - x*x);
	float z = K + 0.5 * y;
	return sqrt(sqrt(z*z - y * INV_ALPHA) - z) * sign(x);
}
float gaussian_rand( vec2 n )
{
    const float FLT_EPSILON = 0.0000001;
	float x = hash12( n );
    x = max( x, FLT_EPSILON );
	return inv_error_function(x*2.0-1.0)*0.15 + 0.5;
}


float dither1( float x, float seed, int dithering_type )
{
    if ( dithering_type == 1 )
    {
        //float t = ( iMouse.z < 0.5 ) ? 0.3 : iMouse.x / iResolution.x;
        //x += (-0.5*t+(1.0+t)*hash11(seed)) / 255.0; //uniform noise
        //x += (-0.15+1.3*hash11(seed)) / 255.0; //uniform noise
        x += hash11(seed) / 255.0; //uniform noise
    }
    else if ( dithering_type == 2 )
        x += (hash11(seed) + hash11(seed+3.1337) - 0.5 ) / 255.0; //triangular noise
    else if ( dithering_type == 3 )
    {
        #ifdef BRUTEFORCE_GAUSSIAN
        float rnd = 0.0;
	 	rnd += hash11( seed + 0.07 );
	 	rnd += hash11( seed + 0.11 );
	 	rnd += hash11( seed + 0.13 );
	 	rnd += hash11( seed + 0.17 );

    	rnd += hash11( seed + 0.19 );
     	rnd += hash11( seed + 0.23 );
     	rnd += hash11( seed + 0.29 );
     	rnd += hash11( seed + 0.31 );
        rnd /= 8.0;
        #else
        float rnd = gaussian_rand( vec2(seed) );
        //TODO: mad 5.25/8.0
        #endif

        #ifdef INPUT_NOISERANGE
        // float t = ( iMouse.z < 0.5 ) ? 3.0/4.0 : iMouse.x / iResolution.x;
        //t = 5.25/8.0;
        // rnd = (1.0 + 4.0 * t) * rnd - 2.0*t; //default to [-1.5;2.5[
        #else
        //rnd = rnd*5.0 - 2.0; // [-2;3[
        rnd = rnd * 4.0 - 1.5; // [-1.5;2.5[
        #endif

        x += rnd / 255.0;
    }
    else
        x += 0.5 / 255.0; //straight rounding

    return x;
}
float dither2( float x, vec2 seed, int dithering_type )
{
    if ( dithering_type == 1 )
    {
        //float t = ( iMouse.z < 0.5 ) ? 0.3 : iMouse.x / iResolution.x;
        //x += (-0.5*t+(1.0+t)*hash12(seed)) / 255.0; //uniform noise
        //x += (-0.15+1.3*hash12(seed)) / 255.0; //uniform noise
        x += hash12(seed) / 255.0; //uniform noise
    }
    else if ( dithering_type == 2 )
        x += (hash12(seed) + hash12(seed+3.1337) - 0.5 ) / 255.0; //triangular noise
    else if ( dithering_type == 3 )
    {
        #ifdef BRUTEFORCE_GAUSSIAN
        float rnd = 0.0;
	 	rnd += hash12( seed + 0.07 );
	 	rnd += hash12( seed + 0.11 );
	 	rnd += hash12( seed + 0.13 );
	 	rnd += hash12( seed + 0.17 );

    	rnd += hash12( seed + 0.19 );
     	rnd += hash12( seed + 0.23 );
     	rnd += hash12( seed + 0.29 );
     	rnd += hash12( seed + 0.31 );
        rnd /= 8.0;
        #else
        float rnd = gaussian_rand( seed );
        //TODO: mad 5.25/8.0
        #endif

        #ifdef INPUT_NOISERANGE
        // float t = ( iMouse.z < 0.5 ) ? 3.0/4.0 : iMouse.x / iResolution.x;
        //t = 5.25/8.0;
        // rnd = (1.0 + 4.0 * t) * rnd - 2.0*t; //note: default to [-1.5;2.5[
        #else
        //rnd = rnd*5.0 - 2.0; // [-2;3[
        rnd = rnd * 4.0 - 1.5; // [-1.5;2.5[
        #endif

        x += rnd / 255.0;
    }
    else
        x += 0.5 / 255.0; //straight rounding

    return x;
}


float eval( float x, float seed, int dithering_type )
{
    float t = x / NUM_LEVELS_F;
    t = dither1( t, seed, dithering_type );
    t = floor( t * 255.0 ) / 255.0;

    return t;
}

vec3 render( vec2 uv, int type )
{
    bool use_uniform = type == 1;
    bool use_triangular = type == 2;
    bool use_gaussian = type == 3;

    //signal
    float s = uv.x;
    #ifdef ANIMATE
    s += 0.5 * (0.5 * sin( iTime ));
    float time = fract( 0.1 * iTime );
    #else
    float time = 0.0;
    #endif

    float v = s / NUM_LEVELS_F;

    vec2 vseed = uv + time;
    v = dither2( v, vseed, type );
    v = floor( v * 255.0 ) / 255.0; //quantisation to 8bit

    vec3 outcol = vec3(v) * NUM_LEVELS_F;
    //return outcol; //DBG dithered signal

    //graph
	if ( uv.y < 1.0/6.0 )
	{
        const int NUM_AVG = 512;
        const float NUM_AVG_F_RCP = 1.0 / float(NUM_AVG);

        vec2 luv = vec2( uv.x, remap(uv.y, 0.0/6.0, 1.0/6.0) );

        // note: running variance calculation
        //https://www.johndcook.com/blog/standard_deviation/
        float m_oldM, m_newM, m_oldS, m_newS;

        float var = 0.0;
        float diffvar = 0.0;
        for ( int i=0; i<NUM_AVG; ++i )
        {
            float seed = s + float(i)*NUM_AVG_F_RCP + time;
            float t = eval( s, seed, type );

            if (i == 0)
            {
                m_oldM = m_newM = t;
                m_oldS = 0.0;
            }
            else
            {
                m_newM = m_oldM + (t - m_oldM) / float(i+1);
                m_newS = m_oldS + (t - m_oldM)*(t - m_newM);

                // set up for next iteration
                m_oldM = m_newM;
                m_oldS = m_newS;
            }
        }

        var = ( (NUM_AVG > 1) ? m_newS / float(NUM_AVG - 1) : 0.0 );

        var *= 255.0 * NUM_LEVELS_F * 9.0; //...really just a random scale-factor, std deviation
        //var *= 255.0 * NUM_LEVELS_F * 0.02; //...really just a random scale-factor, deviation

        diffvar /= float(NUM_AVG-1);
        diffvar = clamp( diffvar, 0.0, 1.0f );

        float stddev = sqrt(var);

        // https://en.wikipedia.org/wiki/Statistical_dispersion
        float oc = step( var, luv.y ); // variance
        //float oc = step( diffvar*250000.0, luv.y ); // error variance
        //float oc = step( stddev, luv.y ); // standard deviation
        //float oc = step( var*var, luv.y ); //...hmm... something, and closer to what I expected...
        //float oc = step( 100.0*abs(mindiff), luv.y ); // min diff to mean
        //float oc = step( 100.0*maxdiff, luv.y ); // max diff to mean?
        //float oc = step( (maxdiff-mindiff)*50.0, luv.y );

        outcol = vec3(0.5 * oc);
        outcol.rgb += 0.125 * step( abs(0.5-luv.y), 6.0/iResolution.y );

        //outcol.g += step( abs(300.0*meanerr-luv.y), 6.0/iResolution.y ); //meanerr
        outcol.r += step( abs(diffvar*250000.0-luv.y), 6.0/iResolution.y ); //diff-var

        //outcol.g += step( abs(mean * NUM_LEVELS_F-luv.y), 6.0/iResolution.y ); //mean
        //outcol.b += 0.5 * step( abs(s-luv.y), 3.0/iResolution.y ); //signal
    }
	else if ( uv.y < 2.0/6.0 )
    {
		//error histogram
        vec2 luv = vec2( uv.x, remap(uv.y, 1.0/6.0, 2.0/6.0) );
        const float minerr = -2.0/255.0;
        const float maxerr = 4.0/255.0;
        const float uvh = 1.0/5.0;
        float uvhpx = uvh * iResolution.y;
        vec2 bucketbounds = ( minerr*uvhpx + maxerr * vec2( luv.y*uvhpx - 1.0, luv.y * uvhpx + 1.0 )) / uvhpx;

        float bucket = 0.0;
        for ( int i=0; i<64; ++i )
        {
            float seed = s + float(i)/64.0 + time;
        	float t = eval( s, seed, type );
            float signal = s/NUM_LEVELS_F;
        	float err = (signal - t); //error
            bucket += float ( err > bucketbounds.x ) * float ( err < bucketbounds.y );
        }

        //outcol = vec3( bucket / 100.0 ) * ((luv.y>0.5) ? vec3(0.5,0.75,1.0) : vec3(1.0,0.5,0.5));
        outcol = vec3( bucket / 100.0 ) * ((luv.y>0.5) ? vec3(0.65,1.0,0.65) : vec3(1.0,0.65,0.65));
        outcol.rgb += step( abs(0.5-luv.y), 3.0 / iResolution.y );
        outcol.rgb += 0.5 * step( abs(0.25-luv.y), 3.0 / iResolution.y );
        outcol.rgb += 0.5 * step( abs(0.75-luv.y), 3.0 / iResolution.y );
    }
    else if ( uv.y < 3.0 / 6.0 )
    {
        //note: error HAS to be two triangles,
        //      because it is either a bit below or above, error of both is triangular
        vec2 luv = vec2( uv.x, remap(uv.y, 2.0/6.0, 3.0/6.0) );
        float seed = s + time;
        float t = eval( s, seed, type );
        //float err = abs(s/NUM_LEVELS_F - t); //abs error
        float signal = s / NUM_LEVELS_F;
        float err = signal - t; //error
        float euv = luv.y * 2.0 - 1.0;
        euv = euv / 255.0;
        if ( euv > 0.0 )
        	outcol = vec3(0.25,0.5,0.25) * vec3( step(euv, err) );
        else
            outcol = vec3(0.5,0.125,0.125) * vec3( step(err, euv) );

        outcol.rgb += 0.5 * step( abs(0.5-luv.y),  6.0/iResolution.y );

        //outcol = vec3( abs(luv), 0.0);
    }
    else if ( uv.y < 4.0 / 6.0 )
    {
        float signal = s / NUM_LEVELS_F; //only show lower part of gradient
        //float err = abs(signal - v ); //error
        float err = signal - v + 1.0 / 255.0;
        outcol = vec3( err * 100.0 );
    }
    else if ( uv.y < 5.0 / 6.0 )
    {
        /*
		//b0rked histogram
        vec2 luv = vec2( uv.x, remap(uv.y, 1.0/6.0, 2.0/6.0) );
        const float uvh = 1.0/5.0;
        float uvhpx = uvh * iResolution.y;

        //float maxval = float(1.0)/255.0;
        float maxval = (1.0+floor(luv.x*8.0)/8.0) / 255.0;

        //return vec3( maxval * 128.0 );

        vec2 bucketbounds = maxval * vec2( luv.y * uvhpx - 1.0, luv.y * uvhpx + 1.0 ) / uvhpx;

        float bucket = 0.0;
        for ( int i=0; i<64; ++i )
        {
        	float t = eval( luv.x, luv.x + float(i)/64.0, type );
        	//oat err = abs(luv.x/NUM_LEVELS_F - t); //error
            bucket += float ( t > bucketbounds.x ) * float ( t < bucketbounds.y );
        }

        outcol = vec3( bucket / 128.0 );

        outcol.g += 0.5 * step( abs(0.5-luv.y), 0.015 );
		*/

        const int NUM_AVG = 1;
        const float NUM_AVG_F = float(NUM_AVG);

        vec2 luv = vec2( uv.x, remap(uv.y, 4.0/6.0, 5.0/6.0) );

        float sum = 0.0;
        for ( int i=0; i<NUM_AVG; ++i )
        {
            float seed = s + float(i)/NUM_AVG_F + time;
            float t = eval( s, seed, type );
        	t *= NUM_LEVELS_F;
            sum += t;
        }
        sum /= NUM_AVG_F;

        float oc = step( sum, luv.y );
        outcol = vec3(0.5 * oc);
        outcol.g += 0.5 * step( abs(s-luv.y), 3.0/iResolution.y ); //signal
    }

    outcol.rgb += vec3( step( abs(mod(uv.y,1.0/6.0)), 2.0 / iResolution.y) );

    return outcol;
}


void main()
{
	vec2 uv = gl_FragCoord.xy / iResolution.xy;

    int idx = int( floor( 2.0*uv.x ) + 2.0 * floor(2.0*uv.y) );


    // if ( iMouse.z > 0.5 )
    // {
    //     vec2 muv = iMouse.xy / iResolution.xy;
    //     idx = int( floor( 2.0*muv.x ) + 2.0 * floor(2.0*muv.y) );
    // }
    #ifndef SINGLE_INDEX
    else
      	uv = fract( 2.0 * uv );
    #endif

    #ifdef SINGLE_INDEX
    idx = SINGLE_INDEX;
    #endif

    outColor = vec4( render( uv, idx ), 1.0 );

    float t = max(step(0.4975, abs(uv.x-0.5)), step(0.495, abs(uv.y-0.5)));

    outColor.rgb = mix( outColor.rgb, vec3(1.0, 0.0, 0.0), t );
    //fragColor.rgb = min( fragColor.rgb, vec3(1.0-t) );
}
