#version 330 core

in vec2 TexCoords;
in vec3 Normal;
in vec3 LightPos[8];
in vec3 ViewPos;
in vec3 FragPos;
in vec3 TangentLightPos[8];
in vec3 TangentViewPos;
in vec3 TangentFragPos;
in mat3 TBN;
in vec3 objPos;

out vec4 color;

uniform sampler2D texture_diffuse1;
uniform int load_normal;
uniform sampler2D texture_normal1;

uniform vec3 lightAmb[8];
uniform vec3 lightDif[8];
uniform int lightType[8];
uniform int use_color;
uniform vec4 color_overwrite;


void main()
{
    vec3 normal = normalize(Normal);

    if (load_normal == 1) {
        vec3 tnormal = texture(texture_normal1, TexCoords).rgb;
        tnormal = normalize(tnormal * 2.0 - 1.0);
        normal = tnormal;
    }

    // loop over all the lights to calculate final color
    vec3 result = vec3(0., 0., 0.);
    for (int i=0; i<8; i++){

        if (lightType[i] == 0){
            continue;
        }

        // get lightDir, same math for point/dir light
        vec3 lightDir = normalize(LightPos[i] - FragPos);
        vec3 lightDis = LightPos[i] - objPos;
        if (load_normal == 1) {
            lightDir = normalize(TangentLightPos[i] - TangentFragPos);
            // lightDis = TangentLightPos[i] - TangentFragPos;
        }
        // vec3 lightDir = normalize(lightDis);

        // diffuse calculation
        float diff = max(dot(normal, lightDir), 0.0);
        vec3 diffuse = diff * lightDif[i];
        // non-conventional way to show diffuse on ambient only side
        if (diff == 0.0){
            diffuse = max(dot(-normal, lightDir) * 0.25, 0.0) * lightDif[i];
        }

        // add ambient to result
        vec3 textureColor = texture(texture_diffuse1, TexCoords).rgb;
        vec3 lightVal = (lightAmb[i] + diffuse) * textureColor;
        
         
        if (lightType[i] == 1){
            // light intensity tapers off ther further the object is
            float dis = lightDis.x*lightDis.x + lightDis.y*lightDis.y + lightDis.z*lightDis.z;
            // dis *= 0.1;
            lightVal /= dis;
        }

        result += lightVal;
    }

    color = vec4(result, 1.0f);
    
    if (use_color == 1) {
        color = color_overwrite;
    }
}