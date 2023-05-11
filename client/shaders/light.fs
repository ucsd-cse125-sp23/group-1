#version 330 core

in vec2 TexCoords;
in vec3 Normal;

out vec4 color;

uniform sampler2D texture_diffuse1;
uniform sampler2D texture_normal1;
uniform vec3 lightAmb;
uniform vec3 lightDif;

void main()
{
    vec3 result = vec3(0.0f, 0.0f, 0.0f);
    vec3 normal = texture(texture_normal1, TexCoords).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    vec3 textureColor = texture(texture_diffuse1, TexCoords).rgb;

    // the essential math
    normal = normalize(Normal * normal);

    // diffuse calculation
    vec3 lightDir = vec3(0.0f, 0.0f, 1.0f);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * lightDif;

    // add ambient to result
    result += (lightAmb + diffuse) * textureColor;

    color = vec4(result, 1.0f);
}