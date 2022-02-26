#include "vbo.h"
#include <iostream>

namespace giz
{
    // float implementation
    VBO<float>::VBO(float data[], unsigned long long count, int bufferType, int usage)
    {
        glGenBuffers(1, &bufferId);
        glBindBuffer(bufferType, bufferId);
        glBufferData(bufferType, count, data, usage);
    }

    // int implementation
    VBO<int>::VBO(int data[], unsigned long long count, int bufferType, int usage)
    {
        glGenBuffers(1, &bufferId);
        glBindBuffer(bufferType, bufferId);
        glBufferData(bufferType, count, data, usage);
    }
}