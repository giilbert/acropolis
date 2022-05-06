#include "VBO.h"
#include <iostream>

namespace giz
{
    // float implementation
    VBO<float>::VBO(float data[], unsigned long long count, int bufferType, int usage)
    {
        glGenBuffers(1, &m_BufferId);
        glBindBuffer(bufferType, m_BufferId);
        glBufferData(bufferType, count, data, usage);
    }

    // int implementation
    VBO<int>::VBO(int data[], unsigned long long count, int bufferType, int usage)
    {
        glGenBuffers(1, &m_BufferId);
        glBindBuffer(bufferType, m_BufferId);
        glBufferData(bufferType, count, data, usage);
    }
}