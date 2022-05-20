#pragma once

// maximum amount of sprites in one batch
#define MAX_BATCH_SIZE 4096

#include <glad/glad.h>
#include <vector>

namespace giz
{
    class SpriteBatch
    {
        unsigned int m_VaoId;

    public:
        SpriteBatch();
        ~SpriteBatch();

        void Draw();
    };
}
