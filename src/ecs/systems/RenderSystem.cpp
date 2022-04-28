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

    meshShader->setMatrix4x4(0, &(currentCamera->projectionMatrix[0][0]));
    meshShader->setMatrix4x4(1, &(currentCamera->entity->transform.toMatrix()[0][0]));

    for (auto renderable : renderables)
    {
        // std::cout << renderable->entity->transform.position.y << "\n";
        meshShader->setMatrix4x4(2, &renderable->entity->transform.toMatrix()[0][0]);
        renderable->draw();
    }
}

void RenderSystem::onWindowSizeChange(int width, int height)
{
    currentCamera->update();
}

void RenderSystem::setCurrentCamera(component::Camera *camera)
{
    currentCamera = camera;
    meshShader->setMatrix4x4(0, &(camera->projectionMatrix[0][0]));
    meshShader->setMatrix4x4(1, &(camera->entity->transform.toMatrix()[0][0]));
}