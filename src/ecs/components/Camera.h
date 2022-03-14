#pragma once

#include "ecs/components/BaseComponent.h"
#include "ecs/systems/RenderSystem.h"
#include <glm/mat4x4.hpp>

namespace giz
{
    namespace component
    {
        class Camera : public component::Base
        {
        public:
            bool requiresUpdate = true;
            // by default, the camera is perspective
            Camera(bool isOrthographic = true);
            virtual Camera::~Camera();

            virtual void init();
            virtual void update();

            glm::mat4 projectionMatrix;
            void makeCurrent();

        private:
            bool isPerspective;
        };
    }
}
