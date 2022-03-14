#pragma once

#include <vector>

#include <glm/mat4x4.hpp>
#include <glm/ext/matrix_transform.hpp>  // glm::translate, glm::rotate, glm::scale
#include <glm/ext/matrix_clip_space.hpp> // glm::perspective
#include <glm/ext/scalar_constants.hpp>  // glm::pi

#include "renderer/Renderable.h"
#include "renderer/Shader.h"
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
            component::Camera *currentCamera = nullptr;

        public:
            // singletons should not be cloneable
            RenderSystem(RenderSystem &other) = delete;
            // singletons should not be assignable
            void operator=(const RenderSystem &) = delete;
            // getter method for the Game singleton
            static RenderSystem *instance();

            std::vector<Renderable *> renderables;
            Shader *meshShader;

            void render();
            void onWindowSizeChange(int width, int height);

            void setCurrentCamera(component::Camera *camera);
        };
    }
}