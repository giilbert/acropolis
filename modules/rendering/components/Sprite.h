#pragma once
#include "ecs/components/BaseComponent.h"
#include <glm/vec4.hpp>

namespace giz
{
    namespace component
    {
        class Sprite : public component::Base
        {
        public:
            virtual void Init();
            virtual void Update();

            glm::vec4 m_Vertices[4];
        };
    }
}