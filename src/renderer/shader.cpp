#include "Shader.h"

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
        giz::logger::logError(shaderInfoBuffer);
    }

    return shader;
}

namespace giz
{
    Shader::Shader(std::string vertexSource, std::string fragmentSource, std::vector<char *> uniforms)
    {
        // creates individual shaders
        unsigned int vertexShader = compileShader(GL_VERTEX_SHADER, vertexSource.c_str());
        unsigned int fragmentShader = compileShader(GL_FRAGMENT_SHADER, fragmentSource.c_str());

        // attach and link the shaders
        unsigned int program = glCreateProgram();
        glAttachShader(program, vertexShader);
        glAttachShader(program, fragmentShader);
        glLinkProgram(program);

        // free up memory
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

    Shader::~Shader()
    {
        glDeleteProgram(program);
    }

    // utility function to read a file to string
    std::string readFileToString(std::string path)
    {
        std::ifstream t(path);
        std::ostringstream sstr;
        sstr << t.rdbuf();
        return sstr.str();
    }

    Shader Shader::loadFromFiles(std::string vertexPath, std::string fragmentPath, std::vector<char *> uniforms)
    {
        // loads the shaders into strings
        std::string vertexSource = readFileToString(vertexPath);
        std::string fragmentSource = readFileToString(fragmentPath);

        return Shader(vertexSource, fragmentSource, uniforms);
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

};