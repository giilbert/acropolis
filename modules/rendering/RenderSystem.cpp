#include "ecs/systems/RenderSystem.h"
#include "ecs/Entity.h"

const char *meshVertexSource = {
#include "generated/rendering/shaders/basic.vert"
};

const char *meshFragmentSource = {
#include "generated/rendering/shaders/basic.frag"
};

const char *spriteVertexSource = {
#include "generated/rendering/shaders/sprite.vert"
};

const char *spriteFragmentSource = {
#include "generated/rendering/shaders/sprite.frag"
};

using giz::Shader;
using giz::systems::RenderSystem;

RenderSystem *RenderSystem::singleton = nullptr;

// init
RenderSystem::RenderSystem()
{
    std::vector<char *> uniforms = {"projectionMatrix", "viewMatrix", "modelMatrix", "time"};
    std::vector<char *> spriteUniforms = {"projectionMatrix", "viewMatrix", "time"};

    // compile shaders
    m_MeshShader = new Shader(meshVertexSource, meshFragmentSource, uniforms);
    m_SpriteShader = new Shader(spriteVertexSource, spriteFragmentSource, spriteUniforms);

    m_SpriteBatches.push_back(new SpriteBatch());
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
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    m_MeshShader->Bind();

    double time = glfwGetTime();
    m_MeshShader->SetFloat(3, time);
    m_MeshShader->SetMatrix4x4(0, &(m_CurrentCamera->m_ProjectionMatrix[0][0]));
    m_MeshShader->SetMatrix4x4(1, &(m_CurrentCamera->m_Entity->m_Transform.m_Matrix[0][0]));

    for (auto renderable : m_Renderables)
    {
        m_MeshShader->SetMatrix4x4(2, &renderable->m_Entity->m_Transform.m_Matrix[0][0]);
        renderable->Draw();
    }

    m_SpriteShader->Bind();
    // m_SpriteShader->SetFloat(2, time);
    m_SpriteShader->SetMatrix4x4(0, &(m_CurrentCamera->m_ProjectionMatrix[0][0]));
    m_SpriteShader->SetMatrix4x4(1, &(m_CurrentCamera->m_Entity->m_Transform.m_Matrix[0][0]));

    for (auto batch : m_SpriteBatches)
    {
        batch->Draw();
    }
}

void RenderSystem::OnWindowSizeChange(int width, int height)
{
    if (m_CurrentCamera != nullptr)
        m_CurrentCamera->Update();
}

void RenderSystem::SetCurrentCamera(component::Camera *camera)
{
    m_CurrentCamera = camera;
}

void RenderSystem::AddSprite(component::Sprite *sprite)
{
    // TODO: overflow - create a new batch
    m_SpriteBatches.back()->AddSprite(sprite);
}