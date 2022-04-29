#include "rendering/components/Camera.h"
#include "ecs/Entity.h"

#include <iostream>

using giz::component::Camera;

Camera::Camera(bool isOrthographic)
{
    isPerspective = !isOrthographic;
}

Camera::~Camera()
{
}

void Camera::init()
{
    update();
}

void Camera::update()
{
    Window *window = Game::instance()->gameWindow;
    float width = window->width;
    float height = window->height;

    projectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, width / height, 0.1f, 1000.0f);
}

void Camera::makeCurrent()
{
    systems::RenderSystem::instance()->setCurrentCamera(this);
}