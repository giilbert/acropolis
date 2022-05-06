#pragma once
#define GLEW_STATIC
#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <iostream>
#include <vector>
#include <map>
#include <fstream>
#include <sstream>
#include "utils/logger.h"

namespace giz
{
    class Shader
    {
    public:
        Shader();
        Shader(std::string vertexSource, std::string fragmentSource, std::vector<char *> uniforms);
        ~Shader();

        static Shader *LoadFromFiles(std::string vertexPath, std::string fragmentPath, std::vector<char *> uniforms);

        void Bind();

        void SetFloat(int idx, float value);
        void SetMatrix4x4(int idx, float *start);

        unsigned int m_Program;
        int *m_UniformLocations;
    };
}