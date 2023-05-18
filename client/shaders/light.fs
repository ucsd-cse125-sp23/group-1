#version 330 core

in vec2 TexCoords;
in vec3 Normal;
in mat3 TBN;

out vec4 color;

uniform sampler2D texture_diffuse1;

uniform int load_normal;
uniform sampler2D texture_normal1;

uniform vec3 lightAmb;
uniform vec3 lightDif;
uniform vec3 lightDir;

void main()
{
    vec3 result = vec3(0.0f, 0.0f, 0.0f);
    vec3 textureColor = texture(texture_diffuse1, TexCoords).rgb;

    vec3 normal = Normal;
    if (load_normal == 1) {
        vec3 tnormal = texture(texture_normal1, TexCoords).rgb;
        tnormal = normalize(tnormal * 2.0 - 1.0);
        normal =  normalize(TBN * tnormal);
    }

    // diffuse calculation
    float diff = max(dot(normal, lightDir), 0.1);
    vec3 diffuse = diff * lightDif;

    // add ambient to result
    result += (lightAmb + diffuse) * textureColor;

    color = vec4(result, 1.0f);
}