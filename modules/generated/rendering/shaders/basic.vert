R"(#version 330

layout(location = 0) in vec3 vertexPosition;
layout(location = 1) in vec3 normal;
out vec3 vNormal;

uniform mat4 modelMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;

void main() {
  gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(vertexPosition, 1.0);
  vNormal = normal;
}
)"