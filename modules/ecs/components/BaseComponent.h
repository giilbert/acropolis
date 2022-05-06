#pragma once

namespace giz
{
    class Entity;
    namespace component
    {
        class Base
        {
        public:
            virtual ~Base() = 0;
            // the instance of an entity which the component belongs to
            Entity *m_Entity;

            // whether or not to also update the component when the entity changes
            bool m_RequiresUpdate = false;

            // register with systems, etc
            virtual void Init() = 0;

            // called when entity changes
            virtual void Update() = 0;
        };
    }
}