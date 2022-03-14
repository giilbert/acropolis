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
        logger::logInfo("Entity destructor called");

        for (auto component : components)
        {
            std::cout << "delete\n";
            delete component;
        }
    }

    void Entity::addComponent(component::Base *component)
    {
        logger::logInfo("Entity init");

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