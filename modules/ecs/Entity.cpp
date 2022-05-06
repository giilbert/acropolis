#include "ecs/Entity.h"

#include "utils/logger.h"

namespace giz
{
    Entity::Entity()
    {
        m_Transform = Transform();
    }

    Entity::~Entity()
    {

        for (auto component : m_Components)
        {
            delete component;
        }
    }

    void Entity::AddComponent(component::Base *component)
    {
        component->m_Entity = this;

        component->Init();
        m_Components.push_back(component);
    }

    void Entity::UpdateComponents()
    {
        for (auto component : m_Components)
        {
            if (component->m_RequiresUpdate)
            {
                component->Update();
            }
        }
    }
}