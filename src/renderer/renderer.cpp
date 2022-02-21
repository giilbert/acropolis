#include "renderer.h"

void glfwError(int code, const char *err)
{
    std::cout << err << "\n";
    glfwTerminate();
}

Renderer::Renderer(GLFWwindow *window)
{
    glfwMakeContextCurrent(window);
    glewInit();
    glfwSetErrorCallback(glfwError);

    glClearColor(0.1, 0.1, 0.1, 0.1);
    glEnable(GL_CULL_FACE);
}

void Renderer::render(GLFWwindow *window)
{
    glClear(GL_COLOR_BUFFER_BIT);

    glDrawElements(GL_TRIANGLES, 36, GL_UNSIGNED_INT, nullptr);

    glfwSwapBuffers(window);
    glfwPollEvents();
}
