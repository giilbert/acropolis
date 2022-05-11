#include "ecs/systems/RenderSystem.h"
#include "ecs/Entity.h"

const char *meshVertexSource = {
#include "generated/rendering/shaders/basic.vert"
};

const char *meshFragmentSource = {
#include "generated/rendering/shaders/basic.frag"
};

using giz::Shader;
using giz::systems::RenderSystem;

RenderSystem *RenderSystem::singleton = nullptr;

// init
RenderSystem::RenderSystem()
{
    std::vector<char *> uniforms = {"projectionMatrix", "viewMatrix", "modelMatrix", "time"};

    // compile shaders
    m_MeshShader = new Shader(meshVertexSource, meshFragmentSource, uniforms);
    // meshShader = Shader::loadFromFiles("res/shaders/basic.vert", "res/shaders/basic.frag", uniforms);
    m_MeshShader->Bind();
}

RenderSystem *RenderSystem::Instance()
{
    if (singleton == nullptr)
    {
        singleton = new RenderSystem();
    }

    return singleton;
}

// renders everything
void RenderSystem::Render()
{
    m_MeshShader->Bind();
    glClear(GL_COLOR_BUFFER_BIT);

    double time = glfwGetTime();
    m_MeshShader->SetFloat(3, time);

    m_MeshShader->SetMatrix4x4(0, &(m_CurrentCamera->m_ProjectionMatrix[0][0]));
    m_MeshShader->SetMatrix4x4(1, &(m_CurrentCamera->m_Entity->m_Transform.m_Matrix[0][0]));

    for (auto renderable : m_Renderables)
    {
        // std::cout << renderable->entity->transform.position.y << "\n";
        m_MeshShader->SetMatrix4x4(2, &renderable->m_Entity->m_Transform.m_Matrix[0][0]);
        renderable->Draw();
    }
}

void RenderSystem::OnWindowSizeChange(int width, int height)
{
    m_CurrentCamera->Update();
}

void RenderSystem::SetCurrentCamera(component::Camera *camera)
{
    m_CurrentCamera = camera;
    m_MeshShader->SetMatrix4x4(0, &(camera->m_ProjectionMatrix[0][0]));
    m_MeshShader->SetMatrix4x4(1, &(camera->m_Entity->m_Transform.m_Matrix[0][0]));
}