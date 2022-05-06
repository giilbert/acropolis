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
            bool m_RequiresUpdate = true;
            // by default, the camera is perspective
            Camera(bool isOrthographic = true);
            virtual Camera::~Camera();

            virtual void Init();
            virtual void Update();

            glm::mat4 m_ProjectionMatrix;
            void MakeCurrent();

        private:
            bool m_IsPerspective;
        };
    }
}
