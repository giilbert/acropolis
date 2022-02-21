#pragma once
#include <vector>
#include <GL/glew.h>
#include <GLFW/glfw3.h>

class Mesh3D
{
public:
    Mesh3D(std::vector<float> vertices, std::vector<unsigned int> indices, std::vector<float> normals);
    unsigned int vaoId;
};