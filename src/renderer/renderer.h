#pragma once
#include <iostream>
#include <GL/glew.h>
#include <GLFW/glfw3.h>

namespace giz
{
    class Renderer
    {
    public:
        Renderer();

        void render(GLFWwindow *window);
    };
}