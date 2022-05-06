#pragma once

#include "ecs/Entity.h"

namespace giz
{
    class Renderable
    {
    public:
        Entity *m_Entity;
        virtual void Draw() = 0;
    };
}