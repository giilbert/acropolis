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

#include "data.h"
#include "renderer/renderer.h"
#include "renderer/shader.h"
#include "renderer/vbo.h"
#include "utils/logger.h"
#include "renderer/3d/mesh.h"

using namespace std::chrono;

void glfwWindowResize(GLFWwindow *window)
{
    int x, y;
    glfwGetWindowSize(window, &x, &y);

    glViewport(0, 0, x, y);
}

// for fps counting
long long then;

int main(void)
{
    logger::logInfo("Hello from giz!");

    GLFWwindow *window;

    if (!glfwInit())
        return -1;

    // create window
    window = glfwCreateWindow(640, 480, "dd", NULL, NULL);
    glfwSetWindowRefreshCallback(window, glfwWindowResize);

    if (!window)
    {
        glfwTerminate();
        return -1;
    }

    Renderer renderer(window);

    // init vertex buffer
    // unsigned int indexBuffer;
    // VBO<float> vertexBuffer(vertices, sizeof(vertices), GL_ARRAY_BUFFER, GL_STATIC_DRAW);
    // VBO<int> indexBuffer(indices, sizeof(indices), GL_ELEMENT_ARRAY_BUFFER, GL_STATIC_DRAW);
    // VBO<float> normalBuffer(normals, sizeof(normals), GL_ARRAY_BUFFER, GL_STATIC_DRAW);

    std::vector<float> vertices(vertexData, vertexData + sizeof(vertexData) / sizeof(vertexData[0]));
    std::vector<unsigned int> indices(indexData, indexData + sizeof(indexData) / sizeof(indexData[0]));
    std::vector<float> normals(normalsData, normalsData + sizeof(normalsData) / sizeof(normalsData[0]));

    Mesh3D mesh(vertices, indices, normals);

    std::vector<char *> uniforms = {"projectionMatrix", "viewMatrix", "time"};

    // compile shaders
    Shader shader = Shader::loadFromFiles("res/shaders/basic.vert", "res/shaders/basic.frag", uniforms);
    shader.bind();

    // set uniform locations
    glm::mat4 projectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, 640.0f / 480.0f, 0.1f, 1000.0f);
    shader.setMatrix4x4(0, &projectionMatrix[0][0]);

    glBindVertexArray(mesh.vaoId);

    float time = 0;
    while (!glfwWindowShouldClose(window))
    {
        glm::mat4 viewMatrix = glm::lookAt(
            glm::vec3(sin(time) * 7, cos(time) * 7, cos(time) * 7),
            glm::vec3(0, 0, 0),
            glm::vec3(0, 1, 0));

        shader.setMatrix4x4(1, &viewMatrix[0][0]);
        shader.setFloat(2, time);

        renderer.render(window);

        time = glfwGetTime();
    }

    glfwTerminate();
    return 0;
}