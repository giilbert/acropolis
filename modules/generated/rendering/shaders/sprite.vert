R"(#version 330

layout(location = 0) in vec4 vertexPosition;

uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

void main() {
    gl_Position = vertexPosition;
}
)"