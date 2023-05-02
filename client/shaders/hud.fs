// FRAGMENT SHADER for HUD
// Renders everything in solid colors (for now)
#version 330 core
out vec4 FragColor;

void main()
{   
    // r, g, b, a
    // set to transparent white
    FragColor = vec4(1.0f, 1.0f, 1.0f, 0.5f);
} 