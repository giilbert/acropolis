#include "ecs/components/Mesh.h"

using giz::component::Mesh;

Mesh::Mesh(std::vector<float> vertices, std::vector<unsigned int> indices, std::vector<float> normals)
{
    mesh = new Mesh3D(vertices, indices, normals);
}

Mesh::~Mesh()
{
    logger::logInfo("Mesh component destructor called");
    delete mesh;
}

void Mesh::init()
{
    mesh->entity = entity;
    systems::RenderSystem::instance()->renderables.push_back(mesh);
}
