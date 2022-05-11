#include "ecs/Transform.h"
#include "ecs/Entity.h"

using giz::Transform;

Transform::Transform()
{
    m_Position = glm::vec3(0.0, 0.0, 0.0);
    m_Rotation = glm::quat(1.0, 0.0, 0.0, 0.0);
    m_Scale = glm::vec3(1.0, 1.0, 1.0);

    UpdateTransform();
}

void Transform::UpdateTransform(glm::mat4 parentMatrix)
{
    glm::mat4 transform = glm::mat4(1.0);
    transform *= glm::toMat4(m_Rotation);
    transform = glm::translate(transform, m_Position);
    transform = glm::scale(transform, m_Scale);

    m_Matrix = parentMatrix * transform;

    // update children
    for (const auto &child : children)
    {
        child->m_Transform.UpdateTransform(m_Matrix);
    }
}