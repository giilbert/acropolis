#pragma once

#include <vector>

#include <glm/mat4x4.hpp>
#include <glm/ext/matrix_transform.hpp>  // glm::translate, glm::rotate, glm::scale
#include <glm/ext/matrix_clip_space.hpp> // glm::perspective
#include <glm/ext/scalar_constants.hpp>  // glm::pi

#include "rendering/Renderable.h"
#include "rendering/Shader.h"
#include "core/Game.h"

namespace giz
{
    namespace component
    {
        class Camera;
    }

    namespace systems
    {
        class RenderSystem
        {

        protected:
            // also init
            RenderSystem();
            static RenderSystem *singleton;
            component::Camera *m_CurrentCamera = nullptr;

        public:
            // singletons should not be cloneable
            RenderSystem(RenderSystem &other) = delete;
            // singletons should not be assignable
            void operator=(const RenderSystem &) = delete;
            // getter method for the Game singleton
            static RenderSystem *Instance();

            std::vector<Renderable *> m_Renderables;
            Shader *m_MeshShader;

            void Render();
            void OnWindowSizeChange(int width, int height);

            void SetCurrentCamera(component::Camera *camera);
        };
    }
}