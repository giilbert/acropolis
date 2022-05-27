#include "rendering/components/Sprite.h"
#include "ecs/systems/RenderSystem.h"
#include <glm/ext.hpp>
#include <glm/gtx/string_cast.hpp>

// 2 0
// 3 1
static const glm::vec4 spriteVertices[] = {
    glm::vec4(1, 1, 0, 1),
    glm::vec4(1, -1, 0, 1),
    glm::vec4(-1, -1, 0, 1),
    glm::vec4(-1, 1, 0, 1)};

using giz::component::Sprite;
using giz::systems::RenderSystem;

void Sprite::Init()
{
    RenderSystem::Instance()->AddSprite(this);
}

void Sprite::Update()
{
    // calculate vertex positions
    m_Vertices[0] = m_Entity->m_Transform.m_Matrix * spriteVertices[0];
    m_Vertices[1] = m_Entity->m_Transform.m_Matrix * spriteVertices[1];
    m_Vertices[2] = m_Entity->m_Transform.m_Matrix * spriteVertices[2];
    m_Vertices[3] = m_Entity->m_Transform.m_Matrix * spriteVertices[3];

    // std::cout << glm::to_string(m_Vertices[0]) << "\n";
}