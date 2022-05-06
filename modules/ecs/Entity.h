#pragma once

#include <vector>
#include "ecs/Transform.h"
#include "ecs/components/BaseComponent.h"

namespace giz
{
    class Entity
    {
    public:
        Entity();
        ~Entity();

        Transform m_Transform;

        int m_Id = 12;

        std::vector<Entity> m_Children;
        std::vector<component::Base *> m_Components;

        void AddComponent(component::Base *component);
        void UpdateComponents();
    };
}