#define GLEW_STATIC
#include <GL/glew.h>

#include <GLFW/glfw3.h>
#include <glm/mat4x4.hpp>
#include <glm/ext/matrix_transform.hpp>  // glm::translate, glm::rotate, glm::scale
#include <glm/ext/matrix_clip_space.hpp> // glm::perspective
#include <glm/ext/scalar_constants.hpp>  // glm::pi

#include <iostream>
#include <chrono>

#include "data.h"
#include "renderer/renderer.h"

using namespace std::chrono;

static const char *vertexSource =
    "#version 330\n"
    "attribute vec3 vertexPosition;\n"
    "attribute vec3 normal;\n"
    "uniform mat4 projectionMatrix;\n"
    "uniform mat4 viewMatrix;\n"
    "varying vec3 vNormal;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = projectionMatrix * viewMatrix * vec4(vertexPosition, 1.0);\n"
    "   vNormal = normal;\n"
    "}\n";

static const char *fragmentSource =
    "#version 330\n"
    "precision mediump float;\n"
    "varying vec3 vNormal;\n"
    "float bias = -0.3;\n"
    "void main()\n"
    "{\n"
    "   vec3 normal = normalize(vNormal);\n"
    "   float light = dot(normal, vec3(0, 1, 0));\n"
    "   light += 1.0 / (light + 1.8) - 0.2;\n"
    "   light = clamp(light, 0.3, 1.0);\n"
    "   gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);\n"
    "   gl_FragColor.rgb *= light;\n"
    "}\n";

void glfwError(int code, const char *err)
{
    std::cout << err << "\n";
    glfwTerminate();
}

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
    GLFWwindow *
        window;

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

    Renderer renderer;
    renderer.draw();

    glfwMakeContextCurrent(window);
    glewInit();
    glfwSetErrorCallback(glfwError);

    glClearColor(0.1, 0.1, 0.1, 0.1);
    glEnable(GL_CULL_FACE);

    unsigned int vertexBuffer, indexBuffer, normalBuffer;
    // init vertex buffer
    glGenBuffers(1, &vertexBuffer);
    glBindBuffer(GL_ARRAY_BUFFER, vertexBuffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    // init index buffer
    glGenBuffers(1, &indexBuffer);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

    glGenBuffers(1, &normalBuffer);
    glBindBuffer(GL_ARRAY_BUFFER, normalBuffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(normals), normals, GL_STATIC_DRAW);

    // compile shaders
    unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexSource, NULL);
    glCompileShader(vertexShader);

    unsigned int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentSource, NULL);
    glCompileShader(fragmentShader);

    unsigned int program = glCreateProgram();
    glAttachShader(program, vertexShader);
    glAttachShader(program, fragmentShader);
    glLinkProgram(program);

    glUseProgram(program);

    // set attribute locations
    glBindBuffer(GL_ARRAY_BUFFER, vertexBuffer);
    int vertexPositionLocation = glGetAttribLocation(program, "vertexPosition");
    glEnableVertexAttribArray(vertexPositionLocation);
    glVertexAttribPointer(vertexPositionLocation, 3, GL_FLOAT, GL_FALSE, sizeof(vertices[0]) * 3, 0);

    glBindBuffer(GL_ARRAY_BUFFER, normalBuffer);
    int vertexNormalLocation = glGetAttribLocation(program, "normal");
    glEnableVertexAttribArray(vertexNormalLocation);
    glVertexAttribPointer(vertexNormalLocation, 3, GL_FLOAT, GL_FALSE, sizeof(normals[0]) * 3, 0);

    // set uniform locations
    glm::mat4 projectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, 640.0f / 480.0f, 0.1f, 1000.0f);
    int projectionMatrixLocation = glGetUniformLocation(program, "projectionMatrix");
    glUniformMatrix4fv(projectionMatrixLocation, 1, GL_FALSE, &projectionMatrix[0][0]);

    int viewMatrixLocation = glGetUniformLocation(program, "viewMatrix");

    float time = 0;
    while (!glfwWindowShouldClose(window))
    {
        glm::mat4 viewMatrix = glm::lookAt(
            glm::vec3(sin(time) * 7, cos(time) * 7, cos(time) * 7),
            glm::vec3(0, 0, 0),
            glm::vec3(0, 1, 0));
        glUniformMatrix4fv(viewMatrixLocation, 1, GL_FALSE, &viewMatrix[0][0]);

        // draws
        glClear(GL_COLOR_BUFFER_BIT);

        glDrawElements(GL_TRIANGLES, 36, GL_UNSIGNED_INT, nullptr);

        glfwSwapBuffers(window);
        glfwPollEvents();

        time += 0.002;
    }

    glfwTerminate();
    return 0;
}