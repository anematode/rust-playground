#version 140

in vec3 my_attr;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

const vec4 ONE = vec4(1.0, 1.0, 1.0, 1.0);

void main() {
    // screen blend mode lol; from wikipedia
    color = ONE - (ONE - vec4(my_attr, 1.0)) * (ONE - texture(tex, v_tex_coords));
}
