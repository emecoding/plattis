#version 330 core
layout (location = 0) in vec4 vertex;

out vec2 TexCoords;

uniform mat4 model;
uniform mat4 projection;

void main()
{
    TexCoords = vertex.zw;
    gl_Position = projection * model * vec4(vertex.xy, 0.0, 0.0);
    //gl_Position = vec4(vertex.x / 800.0 - 1.0, vertex.y / 600 - 1.0, 0.0, 1.0);
}