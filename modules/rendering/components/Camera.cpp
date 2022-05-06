#include "rendering/components/Camera.h"
#include "ecs/Entity.h"

#include <iostream>

using giz::component::Camera;

Camera::Camera(bool isOrthographic)
{
    m_IsPerspective = !isOrthographic;
}

Camera::~Camera()
{
}

void Camera::Init()
{
    Update();
}

void Camera::Update()
{
    Window *window = Game::Instance()->m_GameWindow;
    float width = window->m_Width;
    float height = window->m_Height;

    m_ProjectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, width / height, 0.1f, 1000.0f);
}

void Camera::MakeCurrent()
{
    systems::RenderSystem::Instance()->SetCurrentCamera(this);
}