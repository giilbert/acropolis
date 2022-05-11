#pragma once
#define GLM_PRECISION_HIGHP
#include <glm/vec3.hpp>
#include <glm/mat4x4.hpp>
#include <glm/gtx/quaternion.hpp>
#include <vector>

namespace giz
{
    class Entity;
    class Transform
    {
    public:
        Transform();

        std::vector<giz::Entity *> children;

        glm::vec3 m_Position;
        glm::quat m_Rotation;
        glm::vec3 m_Scale;

        glm::mat4 m_Matrix;
        void UpdateTransform(glm::mat4 parentMatrix = glm::mat4(1.0));
    };
}