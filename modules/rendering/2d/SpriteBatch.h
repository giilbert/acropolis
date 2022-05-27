#pragma once

// maximum amount of sprites in one batch
#define MAX_BATCH_SIZE 4096
#define VERTICES_IN_BATCH MAX_BATCH_SIZE * 4
#define INDICES_IN_BATCH MAX_BATCH_SIZE * 6

#include <glad/glad.h>
#include <list>
#include "rendering/components/Sprite.h"

namespace giz
{
    class SpriteBatch
    {
        std::list<component::Sprite *> m_Sprites;
        unsigned int m_VaoId;
        unsigned int m_VertexBuffer;

    public:
        SpriteBatch();
        ~SpriteBatch();

        void Draw();
        void AddSprite(component::Sprite *sprite);
    };
}
