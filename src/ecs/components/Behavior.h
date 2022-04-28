#pragma once

#include "ecs/components/BaseComponent.h"
#include "ecs/systems/RenderSystem.h"
#include <v8.h>
#include <string>

namespace giz
{
    namespace component
    {
        class Behavior : public component::Base
        {
        public:
            // by default, the camera is perspective
            Behavior(std::string source);
            virtual Behavior::~Behavior();
            virtual void init();
            virtual void update();

            std::string source;
            unsigned int id;
            v8::Persistent<v8::Object> behavior;
        };
    }
}
