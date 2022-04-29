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
            Entity *entity;

            // whether or not to also update the component when the entity changes
            bool requiresUpdate = false;

            // register with systems, etc
            virtual void init() = 0;

            // called when entity changes
            virtual void update() = 0;
        };
    }
}