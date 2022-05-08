#include "Game.h"

namespace giz
{
    Game *Game::singleton = nullptr;

    Game::Game()
    {
    }

    Game *Game::Instance()
    {
        if (singleton == nullptr)
        {
            singleton = new Game();
        }

        return singleton;
    }

    void Game::OnResize(int width, int height)
    {
        m_GameWindow->m_Width = width;
        m_GameWindow->m_Height = height;

        if (width == 0 || height == 0)
            return;

        systems::RenderSystem::Instance()->OnWindowSizeChange(width, height);
        Update();
    }

    void Game::Init()
    {
        using component::Behavior;
        using component::Camera;
        using component::Mesh;

        m_GameWindow = new Window();
        m_GameWindow->Init();

        // init beforehand
        systems::RenderSystem::Instance();
        systems::ScriptingSystem::Instance();

        std::vector<float> vertices(vertexData, vertexData + sizeof(vertexData) / sizeof(vertexData[0]));
        std::vector<unsigned int> indices(indexData, indexData + sizeof(indexData) / sizeof(indexData[0]));
        std::vector<float> normals(normalsData, normalsData + sizeof(normalsData) / sizeof(normalsData[0]));

        auto entityOne = new Entity();
        auto meshOne = new Mesh(vertices, indices, normals);
        entityOne->AddComponent(meshOne);

        auto entityTwo = new Entity();
        entityTwo->m_Transform.m_Position.z = -10;
        auto cameraOne = new Camera();
        entityTwo->AddComponent(cameraOne);
        cameraOne->MakeCurrent();

        // load script file
        std::ifstream stream("test.js");
        std::ostringstream sstr;
        sstr << stream.rdbuf();

        auto behaviorOne = new Behavior(sstr.str());
        entityOne->AddComponent(behaviorOne);

        glClearColor(0.1, 0.1, 0.1, 1.0);

        float x = 0;

        while (!glfwWindowShouldClose(m_GameWindow->m_Window))
        {
            x += 0.001;
            Update();

            entityTwo->UpdateComponents();
        }

        glfwTerminate();

        delete entityOne;
        delete entityTwo;

        systems::ScriptingSystem::Destroy();
    }

    void Game::Update()
    {
        systems::ScriptingSystem::Instance()->UpdateAll();
        systems::RenderSystem::Instance()->Render();

        if (m_KeysPressed[GLFW_KEY_ESCAPE] == true)
        {
            giz::logger::Info("Escape pressed, gracefully exiting");
            glfwSetWindowShouldClose(m_GameWindow->m_Window, 1);
        }

        glfwSwapBuffers(m_GameWindow->m_Window);
        glfwPollEvents();

        m_Time = glfwGetTime();
    }

    void Game::OnCursorMove(double x, double y)
    {
        m_MousePosition.x = x;
        m_MousePosition.y = y;
    }

    void Game::OnKeyPress(int key, int scancode, int action)
    {
        m_KeysPressed[key] = true;
    }

    void Game::onKeyRelease(int key, int scancode, int action)
    {
        m_KeysPressed[key] = false;
    }
}