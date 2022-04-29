#include "ecs/Entity.h"

#include "utils/logger.h"

namespace giz
{
    Entity::Entity()
    {
        transform = Transform();
    }

    Entity::~Entity()
    {

        for (auto component : components)
        {
            delete component;
        }
    }

    void Entity::addComponent(component::Base *component)
    {
        component->entity = this;

        component->init();
        components.push_back(component);
    }

    void Entity::updateComponents()
    {
        for (auto component : components)
        {
            if (component->requiresUpdate)
            {
                component->update();
            }
        }
    }
}