#pragma once
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <iostream>
#include <vector>
#include <map>
#include "../utils/logger.h"

class Shader
{
public:
    Shader(char *vertexSource, char *fragmentSource, std::vector<char *> uniforms);

    void bind();

    void setFloat(int idx, float value);
    void setMatrix4x4(int idx, float *start);

    unsigned int program;
    int *uniformLocations;
};