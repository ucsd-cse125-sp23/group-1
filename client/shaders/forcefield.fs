#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
in float Alpha;

uniform sampler2D texture_diffuse1;

void main()
{
    FragColor = vec4(1.0, 1.0, 1.0, Alpha) * texture(texture_diffuse1, TexCoords);
}
