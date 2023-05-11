#version 330 core

struct PointLight {
    vec3 position;    
    vec3 ambient;
    vec3 diffuse;
};

#define NR_POINT_LIGHTS 1

in vec3 fragPosition;
in vec3 Normal;
in vec2 TexCoords;

out vec4 color;

uniform sampler2D texture_diffuse1;
uniform sampler2D texture_normal1;
uniform PointLight pointLights[NR_POINT_LIGHTS];

void main()
{
    vec3 result = vec3(0.0f, 0.0f, 0.0f);
    vec3 norm = normalize(vec3(texture(texture_normal1, TexCoords)));
    vec3 norm = Normal;
    vec3 textureColor = vec3(texture(texture_diffuse1, TexCoords));

    for(int i = 0; i < NR_POINT_LIGHTS; i++){
        // diffuse calculation
        vec3 lightDir = vec3(0.0f, 0.0f, 1.0f);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * pointLights[i].diffuse;

        // add ambient to result
        result += (pointLights[i].ambient + diffuse) * textureColor;
    }

    color = vec4(result, 1.0f);
}