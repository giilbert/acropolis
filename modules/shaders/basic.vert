#version 330
layout(location = 0) in vec3 vertexPosition;
layout(location = 1) in vec3 normal;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
out vec3 vNormal;

void main()
{
  gl_Position = projectionMatrix * viewMatrix * vec4(vertexPosition, 1.0);
  vNormal = normal;
};