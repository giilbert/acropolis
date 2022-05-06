#include "rendering/components/Mesh.h"

using giz::component::Mesh;

Mesh::Mesh(std::vector<float> vertices, std::vector<unsigned int> indices, std::vector<float> normals)
{
    m_Mesh = new Mesh3D(vertices, indices, normals);
}

Mesh::~Mesh()
{
    delete m_Mesh;
}

void Mesh::Init()
{
    m_Mesh->m_Entity = m_Entity;
    systems::RenderSystem::Instance()->m_Renderables.push_back(m_Mesh);
}

void Mesh::Update()
{
}