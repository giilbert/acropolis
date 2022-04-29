#pragma once

#include "ecs/systems/RenderSystem.h"
#include "ecs/components/BaseComponent.h"
#include "rendering/3d/Mesh3D.h"
#include "GL/glew.h"
#include "GLFW/glfw3.h"
#include "data.h"

namespace giz
{
    namespace component
    {
        class Mesh : public component::Base
        {
        public:
            Mesh(std::vector<float> vertices, std::vector<unsigned int> indices, std::vector<float> normals);
            virtual Mesh::~Mesh();

            unsigned int vaoId;
            Mesh3D *mesh;

            std::vector<float> vertices;
            std::vector<unsigned int> indices;
            std::vector<float> normals;

            virtual void init();
            virtual void update();
        };
    }
}
