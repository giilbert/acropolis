#pragma once
#include <iostream>
#include <GL/glew.h>
#include <GLFW/glfw3.h>

class Renderer
{
public:
    Renderer(GLFWwindow *window);

    void ready();
    void render(GLFWwindow *window);
};