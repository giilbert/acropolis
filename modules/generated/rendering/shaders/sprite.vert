R"(#version 330

layout(location = 0) in vec2 vertexPosition;

uniform float time;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

void main() {
    gl_Position = projectionMatrix * vec4(vertexPosition, 0.0, 1.0);
}
)"