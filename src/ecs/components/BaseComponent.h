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

            // register with systems, etc
            virtual void init() = 0;
        };
    }
}