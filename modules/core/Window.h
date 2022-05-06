#pragma once

#include <glad/glad.h>
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

        GLFWwindow *m_Window;
        void MakeContextCurrent();
        void Init();

        int m_Width;
        int m_Height;
    };
}