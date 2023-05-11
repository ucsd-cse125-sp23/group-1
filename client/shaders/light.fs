#version 330 core

struct PointLight {
    vec3 position;    
    vec3 ambient;
    vec3 diffuse;
};

#define NR_POINT_LIGHTS 1

in vec2 TexCoords;
in vec3 fragPosition;
in vec3 Normal;
in vec3 TangentLightPos;
in vec3 TangentViewPos;
in vec3 TangentFragPos;

out vec4 color;

uniform sampler2D texture_diffuse1;
uniform sampler2D texture_normal1;
// uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform vec3 lightAmb;
uniform vec3 lightDif;

void main()
{
    vec3 result = vec3(0.0f, 0.0f, 0.0f);
    vec3 normal = texture(texture_normal1, TexCoords).rgb;
    normal = normalize(normal * 2.0 - 1.0);
    vec3 textureColor = texture(texture_diffuse1, TexCoords).rgb;

    normal = normalize(Normal * normal);

    // for(int i = 0; i < NR_POINT_LIGHTS; i++){

    // diffuse calculation
    vec3 lightDir = vec3(0.0f, 0.0f, 1.0f);
    // vec3 lightDir = normalize(TangentLightPos - TangentFragPos);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * lightDif;

    // add ambient to result
    result += (lightAmb + diffuse) * textureColor;

    // }

    color = vec4(result, 1.0f);
}