#pragma once
#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <vector>

#include "rendering/Renderable.h"

namespace giz
{
    class Mesh3D : virtual public Renderable
    {
    public:
        Mesh3D(std::vector<float> vertices, std::vector<unsigned int> indices, std::vector<float> normals);
        ~Mesh3D();

        unsigned int vaoId;

        virtual void draw();
    };
}