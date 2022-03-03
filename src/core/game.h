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
#include <map>

#include "data.h"
#include "renderer/Renderer.h"
#include "renderer/Shader.h"
#include "renderer/VBO.h"
#include "utils/logger.h"
#include "renderer/3d/Mesh.h"
#include "core/Window.h"

namespace giz
{
    class Window;
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
        Window *gameWindow;

        // window events
        void onResize(int width, int height);
        void onCursorMove(double x, double y);
        void onKeyPress(int key, int scancode, int action);
        void onKeyRelease(int key, int scancode, int action);

        void init();
        void update();

    private:
        // time in seconds passed since init() was called
        float time;

        // events
        glm::dvec2 mousePosition;
        std::map<int, bool> keysPressed;
    };
}
