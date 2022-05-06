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
    void Window::Init()
    {
        if (!glfwInit())
        {
            logger::Error("GLFW failed to init");
            exit(-1);
        }

        // create window
        m_Window = glfwCreateWindow(640, 480, "dd", nullptr, nullptr);
        m_Width = 640;
        m_Height = 480;
        glfwSetWindowRefreshCallback(m_Window, glfwWindowResize);

        if (!m_Window)
        {
            logger::Error("Window failed to create");
            glfwTerminate();
            exit(-1);
        }

        glfwSetWindowSizeCallback(
            m_Window, [](GLFWwindow *window, int width, int height)
            {
                glViewport(0, 0, width, height);
                Game::Instance()->OnResize(width, height); });

        glfwSetCursorPosCallback(
            m_Window, [](GLFWwindow *window, double x, double y)
            { Game::Instance()->OnCursorMove(x, y); });

        glfwSetKeyCallback(
            m_Window, [](GLFWwindow *window, int key, int scancode, int action, int mod)
            { 
                switch (action) {
                case GLFW_PRESS:
                    Game::Instance()->OnKeyPress(key, scancode, mod);
                    break;
                case GLFW_RELEASE:
                    Game::Instance()->onKeyRelease(key, scancode, mod);
                    break;
                // TODO: GLFW_REPEAT?
            } });

        // TODO: JOYSTICK & other event callbacks

        glfwMakeContextCurrent(m_Window);

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