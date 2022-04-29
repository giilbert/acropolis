#include "Window.h"

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
    // TODO: init with width and height
    void Window::init()
    {
        if (!glfwInit())
        {
            logger::logError("GLFW failed to init");
            exit(-1);
        }

        // create window
        window = glfwCreateWindow(640, 480, "dd", nullptr, nullptr);
        width = 640;
        height = 480;
        glfwSetWindowRefreshCallback(window, glfwWindowResize);

        if (!window)
        {
            logger::logError("Window failed to create");
            glfwTerminate();
            exit(-1);
        }

        glfwSetWindowSizeCallback(
            window, [](GLFWwindow *window, int width, int height)
            {
                glViewport(0, 0, width, height);
                Game::instance()->onResize(width, height); });

        glfwSetCursorPosCallback(
            window, [](GLFWwindow *window, double x, double y)
            { Game::instance()->onCursorMove(x, y); });

        glfwSetKeyCallback(
            window, [](GLFWwindow *window, int key, int scancode, int action, int mod)
            { 
                switch (action) {
                case GLFW_PRESS:
                    Game::instance()->onKeyPress(key, scancode, mod);
                    break;
                case GLFW_RELEASE:
                    Game::instance()->onKeyRelease(key, scancode, mod);
                    break;
                // TODO: GLFW_REPEAT?
            } });

        // TODO: JOYSTICK & other event callbacks

        glfwMakeContextCurrent(window);

        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
        {
            std::cout << "Failed to initialize OpenGL context" << std::endl;
            exit(-1);
        }

        glfwSetErrorCallback(glfwError);

        glClearColor(0.1, 0.1, 0.1, 0.1);
        glEnable(GL_CULL_FACE);
    }
}