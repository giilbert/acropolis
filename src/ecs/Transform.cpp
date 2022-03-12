#include "ecs/Transform.h"

using giz::Transform;

Transform::Transform()
{
    position = glm::vec3(0.0, 0.0, 0.0);
    rotation = glm::quat(1.0, 0.0, 0.0, 0.0);
    scale = glm::vec3(1.0, 1.0, 1.0);
}

glm::mat4 Transform::toMatrix()
{
    glm::mat4 transform = glm::mat4(1.0);
    transform = glm::translate(transform, position);
    transform *= glm::toMat4(rotation);
    transform = glm::scale(transform, scale);

    return transform;
}
