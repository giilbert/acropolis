#pragma once
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include "utils/logger.h"

namespace giz
{
    template <typename T>
    class VBO
    {
    public:
        unsigned int bufferId;

        VBO(float data[], unsigned long long size, int bufferType, int usage);
        VBO(int data[], unsigned long long size, int bufferType, int usage);
        // VBO(unsigned int data[], unsigned long long size, GLenum bufferType, GLenum usage);
    };
}