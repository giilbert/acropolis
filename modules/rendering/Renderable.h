#pragma once

#include "ecs/Entity.h"

namespace giz
{
    class Renderable
    {
    public:
        Entity *entity;
        virtual void draw() = 0;
    };
}