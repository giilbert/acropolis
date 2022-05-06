#pragma once

#include "ecs/systems/RenderSystem.h"
#include "ecs/components/BaseComponent.h"
#include "rendering/3d/Mesh3D.h"
#include <glad/glad.h>
#include <GLFW/glfw3.h>
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

            unsigned int m_VaoId;
            Mesh3D *m_Mesh;

            std::vector<float> m_Vertices;
            std::vector<unsigned int> m_Indices;
            std::vector<float> m_Normals;

            virtual void Init();
            virtual void Update();
        };
    }
}
