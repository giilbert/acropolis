#include "rendering/components/Mesh.h"

using giz::component::Mesh;

Mesh::Mesh(std::vector<float> vertices, std::vector<unsigned int> indices, std::vector<float> normals)
{
    mesh = new Mesh3D(vertices, indices, normals);
}

Mesh::~Mesh()
{
    delete mesh;
}

void Mesh::init()
{
    mesh->entity = entity;
    systems::RenderSystem::instance()->renderables.push_back(mesh);
}

void Mesh::update()
{
}