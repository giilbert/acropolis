#pragma once

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include "utils/logger.h"
#include "core/Game.h"

namespace giz
{
    // reference to a GLFWwindow, encapsulated with a Window class
    class Window
    {
    public:
        Window();

        GLFWwindow *window;
        void makeContextCurrent();
        void init();
    };
}