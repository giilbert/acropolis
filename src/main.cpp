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

char *vertexSource =
    "#version 330\n"
    "layout(location = 0) in vec3 vertexPosition;\n"
    "layout(location = 1) in vec3 normal;\n"
    "uniform mat4 projectionMatrix;\n"
    "uniform mat4 viewMatrix;\n"
    "out vec3 vNormal;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = projectionMatrix * viewMatrix * vec4(vertexPosition, 1.0);\n"
    "   vNormal = normal;\n"
    "}\n";

char *fragmentSource =
    "#version 330\n"
    "precision mediump float;\n"
    "in vec3 vNormal;\n"
    "uniform float time;\n"
    "float bias = -0.3;\n"
    "float PI = 3.1415926535;\n"
    "vec3 hsb2rgb( in vec3 c ){\n"
    "   vec3 rgb = clamp(abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),\n"
    "                        6.0)-3.0)-1.0,\n"
    "                0.0,\n"
    "                1.0 );\n"
    "   rgb = rgb*rgb*(3.0-2.0*rgb);\n"
    "   return c.z * mix(vec3(1.0), rgb, c.y);\n"
    "}\n"
    "void main()\n"
    "{\n"
    "   vec3 normal = normalize(vNormal);\n"
    "   float light = dot(normal, vec3(0, 1, 0.5));\n"
    "   light += 1.0 / (light + 1.8) - 0.2;\n"
    "   light = clamp(light, 0.3, 1.0);\n"
    "   gl_FragColor = vec4(hsb2rgb(vec3(time / 10.0, 0.7, 0.66)), 1.0);\n"
    "   gl_FragColor.rgb *= light;\n"
    "}\n";

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

    std::vector<char *>
        uniforms = {"projectionMatrix", "viewMatrix", "time"};

    // compile shaders
    Shader shader = Shader(vertexSource, fragmentSource, uniforms);
    shader.bind();
    unsigned int program = shader.program;

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

        // draws

        time = glfwGetTime();
    }

    glfwTerminate();
    return 0;
}