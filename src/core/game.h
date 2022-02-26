#pragma once
#define GLEW_STATIC

#include <GL/glew.h>
#include <GLFW/glfw3.h>

#include <glm/mat4x4.hpp>
#include <glm/ext/matrix_transform.hpp>  // glm::translate, glm::rotate, glm::scale
#include <glm/ext/matrix_clip_space.hpp> // glm::perspective
#include <glm/ext/scalar_constants.hpp>  // glm::pi

#include <iostream>
#include <chrono>
#include <vector>

#include "data.h"
#include "renderer/renderer.h"
#include "renderer/shader.h"
#include "renderer/vbo.h"
#include "utils/logger.h"
#include "renderer/3d/mesh.h"
#include "window.h"

namespace giz
{
    class Game
    {
    protected:
        Game();
        static Game *singleton;

    public:
        // singletons should not be cloneable
        Game(Game &other) = delete;
        // singletons should not be assignable
        void operator=(const Game &) = delete;
        // getter method for the Game singleton
        static Game *instance();

        Renderer mainRenderer;
        Window gameWindow;
        // time in seconds passed since init() was called
        float time;

        void init();
        void update();
    };
}