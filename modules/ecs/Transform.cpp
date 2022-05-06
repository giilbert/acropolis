#include "ecs/Transform.h"

using giz::Transform;

Transform::Transform()
{
    m_Position = glm::vec3(0.0, 0.0, 0.0);
    m_Rotation = glm::quat(1.0, 0.0, 0.0, 0.0);
    m_Scale = glm::vec3(1.0, 1.0, 1.0);
}

glm::mat4 Transform::ToMatrix()
{
    glm::mat4 transform = glm::mat4(1.0);
    transform *= glm::toMat4(m_Rotation);
    transform = glm::translate(transform, m_Position);
    transform = glm::scale(transform, m_Scale);

    return transform;
}
