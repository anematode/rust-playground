#version 140

in vec3 position;
in vec2 tex_coords;
// Pass attribute to vertex shader
out vec3 my_attr;
out vec2 v_tex_coords;

uniform mat4 matrix;
uniform mat4 perspective;

void main() {
    v_tex_coords = tex_coords;
    my_attr = position;
    gl_Position = perspective * matrix * vec4(position, 1.0);
}
