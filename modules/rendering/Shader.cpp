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
        giz::logger::Error(shaderInfoBuffer);
    }

    return shader;
}

namespace giz
{
    // default constructor
    Shader::Shader() {}

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

        this->m_Program = program;

        // uniforms are inserted based on their order when pushed into the uniforms vector
        m_UniformLocations = new int[uniforms.size()];
        for (int i = 0; i < uniforms.size(); i++)
        {
            m_UniformLocations[i] = glGetUniformLocation(program, uniforms[i]);
        }
    }

    Shader::~Shader()
    {
        glDeleteProgram(m_Program);
    }

    // utility function to read a file to string
    std::string readFileToString(std::string path)
    {
        std::ifstream t(path);
        std::ostringstream sstr;
        sstr << t.rdbuf();
        return sstr.str();
    }

    Shader *Shader::LoadFromFiles(std::string vertexPath, std::string fragmentPath, std::vector<char *> uniforms)
    {
        // loads the shaders into strings
        std::string vertexSource = readFileToString(vertexPath);
        std::string fragmentSource = readFileToString(fragmentPath);

        return new Shader(vertexSource, fragmentSource, uniforms);
    }

    void Shader::Bind()
    {
        glUseProgram(this->m_Program);
    }

    void Shader::SetFloat(int idx, float value)
    {
        glUniform1f(idx, value);
    }

    void Shader::SetMatrix4x4(int idx, float *start)
    {
        glUniformMatrix4fv(m_UniformLocations[idx], 1, GL_FALSE, start);
    }

};