#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D atlas;

void main() {
    color = texture(atlas, v_tex_coords);
}
