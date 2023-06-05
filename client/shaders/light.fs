#version 330 core

in vec2 TexCoords;
in vec3 Normal;
in vec3 TangentLightPos;
in vec3 TangentViewPos;
in vec3 TangentFragPos;
in mat3 TBN;

out vec4 color;

uniform sampler2D texture_diffuse1;
uniform int load_normal;
uniform sampler2D texture_normal1;

uniform vec3 lightAmb;
uniform vec3 lightDif;

void main()
{
    vec3 normal = Normal;
    vec3 lightDir = vec3(0.f, 0.f, 1.f);

    if (load_normal == 1) {
        vec3 tnormal = texture(texture_normal1, TexCoords).rgb;
        tnormal = normalize(tnormal * 2.0 - 1.0);
        normal = tnormal;

        lightDir = normalize(TangentLightPos - TangentFragPos);
    }

    // diffuse calculation
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * lightDif;
    // non-conventional way to show diffuse on ambient only side
    if (diff == 0.0){
        diffuse = max(dot(-normal, lightDir) * 0.25, 0.0) * lightDif;
    }

    // add ambient to result
    vec3 textureColor = texture(texture_diffuse1, TexCoords).rgb;
    vec3 result = (lightAmb + diffuse) * textureColor;
    color = vec4(result, 1.0f);
}