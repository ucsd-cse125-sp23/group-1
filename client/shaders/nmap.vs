#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in vec3 aTangent;
layout (location = 4) in vec3 aBitangent;

out vec2 TexCoords;
out vec3 Normal;
out vec3 TangentLightPos;
out vec3 TangentViewPos;
out vec3 TangentFragPos;
out mat3 TBN;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 viewPos;
uniform vec3 lightDir;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
    TexCoords = aTexCoords;

    mat3 normalMatrix = mat3(transpose(inverse(mat3(model))));
    Normal = normalMatrix * aNormal;

    vec3 T = normalize(vec3(model * vec4(aTangent,   1.0f)));
    vec3 B = normalize(vec3(model * vec4(aBitangent, 1.0f)));
    vec3 N = normalize(vec3(model * vec4(aNormal,    1.0f)));
    TBN = mat3(T, B, N);

    vec3 pos = vec3(model * vec4(aPos, 1.0f));
    TangentLightPos = TBN * (lightDir + pos);
    TangentViewPos  = TBN * viewPos;
    TangentFragPos  = TBN * vec3(pos);
}