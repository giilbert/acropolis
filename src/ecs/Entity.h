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

        Transform transform;

        std::vector<Entity> children;
        std::vector<component::Base *> components;

        void addComponent(component::Base *component);
    };
}