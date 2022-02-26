#include "window.h"

void glfwWindowResize(GLFWwindow *window)
{
    int x, y;
    glfwGetWindowSize(window, &x, &y);

    glViewport(0, 0, x, y);
}

void glfwError(int code, const char *err)
{
    std::cout << err << "\n";
    glfwTerminate();
}

namespace giz
{
    Window::Window()
    {
    }

    void Window::init()
    {
        if (!glfwInit())
        {
            logger::logError("GLFW failed to init");
            exit(-1);
        }

        // create window
        window = glfwCreateWindow(640, 480, "dd", nullptr, nullptr);
        glfwSetWindowRefreshCallback(window, glfwWindowResize);

        if (!window)
        {
            logger::logError("Window failed to create");
            glfwTerminate();
            exit(-1);
        }

        glfwMakeContextCurrent(window);
        glewInit();
        glfwSetErrorCallback(glfwError);

        glClearColor(0.1, 0.1, 0.1, 0.1);
        glEnable(GL_CULL_FACE);
    }
}