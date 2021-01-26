#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;

layout(location = 0) out vec3 vPos;
const vec2 positions[3] = vec2[3](
    vec2(-1., 3.),
    vec2(-1., -1.),
    vec2(3., -1.)
);

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    // vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1 *.5;
    // vPos = vec3(positions[gl_VertexIndex], 1.);
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    gl_Position = vec4(a_position * 2., 1.);
}
