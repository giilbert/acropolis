#include "shader.h"

const int aVertexPositionLocation = 0;
const int aNormalLocation = 1;

unsigned int compileShader(int type, const char *source)
{
    unsigned int shader = glCreateShader(type);
    glShaderSource(shader, 1, &source, NULL);
    glCompileShader(shader);

    int length;
    glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &length);

    char *shaderInfoBuffer = new char[length];

    glGetShaderInfoLog(shader, 500, &length, shaderInfoBuffer);

    if (length != 0)
    {
        logger::logError(shaderInfoBuffer);
    }

    return shader;
}

Shader::Shader(char *vertexSource, char *fragmentSource, std::vector<char *> uniforms)
{
    unsigned int vertexShader = compileShader(GL_VERTEX_SHADER, vertexSource);
    unsigned int fragmentShader = compileShader(GL_FRAGMENT_SHADER, fragmentSource);

    unsigned int program = glCreateProgram();
    glAttachShader(program, vertexShader);
    glAttachShader(program, fragmentShader);
    glLinkProgram(program);

    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    this->program = program;

    // uniforms are inserted based on their order when pushed into the uniforms vector
    uniformLocations = new int[uniforms.size()];
    for (int i = 0; i < uniforms.size(); i++)
    {
        uniformLocations[i] = glGetUniformLocation(program, uniforms[i]);
    }
}

void Shader::bind()
{
    glUseProgram(this->program);
}

void Shader::setFloat(int idx, float value)
{
    glUniform1f(idx, value);
}

void Shader::setMatrix4x4(int idx, float *start)
{
    glUniformMatrix4fv(uniformLocations[idx], 1, GL_FALSE, start);
}