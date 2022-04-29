#pragma once
#define GLM_PRECISION_HIGHP
#include <glm/vec3.hpp>
#include <glm/mat4x4.hpp>
#include <glm/gtx/quaternion.hpp>

namespace giz
{
    class Transform
    {
    public:
        Transform();
        glm::vec3 position;
        glm::quat rotation;
        glm::vec3 scale;

        glm::mat4 toMatrix();
    };
}