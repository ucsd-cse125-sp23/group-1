#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in vec3 aTangent;
layout (location = 4) in vec3 aBitangent;

out vec2 TexCoords;
out vec3 Normal;
out vec3 LightPos[4];
out vec3 ViewPos;
out vec3 FragPos;
out vec3 TangentLightPos[4];
out vec3 TangentViewPos;
out vec3 TangentFragPos;
out mat3 TBN;

uniform mat4 model;
uniform mat4 model_scaleless;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 viewPos;

uniform vec3 lightDir[4];
uniform int lightType[4];

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
    TexCoords = aTexCoords;

    vec3 T = normalize(vec3(model_scaleless * vec4(aTangent,   0.0)));
    vec3 B = normalize(vec3(model_scaleless * vec4(aBitangent, 0.0)));
    vec3 N = normalize(vec3(model_scaleless * vec4(aNormal,    0.0)));
    TBN = transpose(mat3(T, B, N));

    vec3 pos = vec3(model_scaleless * vec4(aPos, 1.0f));
    ViewPos  = viewPos;
    FragPos  = pos;
    TangentViewPos  = TBN * viewPos;
    TangentFragPos  = TBN * pos;
    
    // setup the light positions
    for (int i=0; i<4; i++){

        // assume its point light, change to directional light if nec
        LightPos[i] = lightDir[i];
        if (lightType[i] == 2) {
            LightPos[i] += pos;
        }

        TangentLightPos[i] = TBN * LightPos[i];
    }

    mat3 normalMatrix = transpose(inverse(mat3(model_scaleless)));
    Normal = mat3(normalMatrix) * aNormal;
}