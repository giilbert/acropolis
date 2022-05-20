#include "rendering/components/Camera.h"
#include "ecs/Entity.h"

#include <iostream>

using giz::component::Camera;

Camera::Camera()
{
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

    if (m_Projection == Projection::Perspective)
    {
        // perspective
        m_ProjectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, width / height, 0.01f, 1000.0f);
    }
    else
    {
        float aspectRatio = width / height;
        // orthographic
        m_ProjectionMatrix = glm::ortho(-m_Size * aspectRatio, m_Size * aspectRatio, -m_Size, m_Size, 0.0f, 1000.0f);
    }
}

void Camera::SetProjection(Projection projection)
{
    m_Projection = projection;
    Update();
}

void Camera::MakeCurrent()
{
    systems::RenderSystem::Instance()->SetCurrentCamera(this);
}