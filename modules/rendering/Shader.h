#pragma once
#define GLEW_STATIC
#include <GL/glew.h>
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

        static Shader *loadFromFiles(std::string vertexPath, std::string fragmentPath, std::vector<char *> uniforms);

        void bind();

        void setFloat(int idx, float value);
        void setMatrix4x4(int idx, float *start);

        unsigned int program;
        int *uniformLocations;
    };
}