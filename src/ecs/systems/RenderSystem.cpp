#include "ecs/systems/RenderSystem.h"

using giz::Shader;
using giz::systems::RenderSystem;

RenderSystem *RenderSystem::singleton = nullptr;

// init
RenderSystem::RenderSystem()
{
    std::vector<char *> uniforms = {"projectionMatrix", "viewMatrix", "modelMatrix", "time"};

    // compile shaders
    meshShader = Shader::loadFromFiles("res/shaders/basic.vert", "res/shaders/basic.frag", uniforms);
    meshShader->bind();
}

RenderSystem *RenderSystem::instance()
{
    if (singleton == nullptr)
    {
        singleton = new RenderSystem();
    }

    return singleton;
}

// renders everything
void RenderSystem::render()
{
    // meshShader.bind();
    glClear(GL_COLOR_BUFFER_BIT);

    double time = glfwGetTime();
    meshShader->setFloat(3, time);

    glm::mat4 projectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, (float)640 / (float)480, 0.1f, 1000.0f);
    meshShader->setMatrix4x4(0, &projectionMatrix[0][0]);

    glm::mat4 viewMatrix = glm::lookAt(
        glm::vec3(5, 5, 5),
        glm::vec3(0, 0, 0),
        glm::vec3(0, 1, 0));

    meshShader->setMatrix4x4(1, &viewMatrix[0][0]);

    for (auto renderable : renderables)
    {
        renderable->entity->transform.position.y = glm::sin(time);
        meshShader->setMatrix4x4(2, &renderable->entity->transform.toMatrix()[0][0]);
        renderable->draw();
    }
}