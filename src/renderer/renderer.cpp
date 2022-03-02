#include "Renderer.h"

namespace giz
{
    Renderer::Renderer(){};

    void Renderer::render(GLFWwindow *window)
    {
        glClear(GL_COLOR_BUFFER_BIT);

        glDrawElements(GL_TRIANGLES, 36, GL_UNSIGNED_INT, nullptr);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }
}