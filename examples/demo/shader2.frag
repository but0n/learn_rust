#version 450

layout(location = 0) in vec3 vPos;

layout(location = 0) out vec4 outColor;

layout(set=0, binding=0)
uniform Uniforms {
    float u_Time;
};

#define GAMMA 2.2
// Tone map
vec3 toneMapACES(vec3 color) {
    const float A = 2.51;
    const float B = 0.03;
    const float C = 2.43;
    const float D = 0.59;
    const float E = 0.14;
    return pow(clamp((color * (A * color + B)) / (color * (C * color + D) + E), 0.0, 1.0), vec3(1.0/GAMMA));
}

void main() {
    float t = u_Time * .02;
    vec3 coord = gl_FragCoord.xyz * vec3(sin(t), cos(t), 0);
    vec3 color = vec3(coord / vec3(2048., 1440., 1.));
    outColor = vec4(1. - color, 1.0);
    outColor = vec4(toneMapACES(outColor.rgb), 1.);
}
