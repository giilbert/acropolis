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
        glm::vec3 m_Position;
        glm::quat m_Rotation;
        glm::vec3 m_Scale;

        glm::mat4 ToMatrix();
    };
}