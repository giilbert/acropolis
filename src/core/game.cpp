#include "Game.h"

namespace giz
{
    Game *Game::singleton = nullptr;

    Game::Game()
    {
    }

    Game *Game::instance()
    {
        if (singleton == nullptr)
        {
            singleton = new Game();
        }

        return singleton;
    }

    void Game::onResize(int width, int height)
    {
        gameWindow->width = width;
        gameWindow->height = height;
        systems::RenderSystem::instance()->onWindowSizeChange(width, height);
    }

    void Game::init()
    {
        using component::Behavior;
        using component::Camera;
        using component::Mesh;

        gameWindow = new Window();
        gameWindow->init();

        // init beforehand
        systems::RenderSystem::instance();
        systems::ScriptingSystem::instance();

        std::vector<float> vertices(vertexData, vertexData + sizeof(vertexData) / sizeof(vertexData[0]));
        std::vector<unsigned int> indices(indexData, indexData + sizeof(indexData) / sizeof(indexData[0]));
        std::vector<float> normals(normalsData, normalsData + sizeof(normalsData) / sizeof(normalsData[0]));

        auto entityOne = new Entity();
        auto meshOne = new Mesh(vertices, indices, normals);
        entityOne->addComponent(meshOne);

        auto entityTwo = new Entity();
        entityTwo->transform.position.z = -10;
        auto cameraOne = new Camera();
        entityTwo->addComponent(cameraOne);
        cameraOne->makeCurrent();

        // load script file
        std::ifstream stream("test.js");
        std::ostringstream sstr;
        sstr << stream.rdbuf();

        auto behaviorOne = new Behavior(sstr.str());
        entityOne->addComponent(behaviorOne);

        glClearColor(0.1, 0.1, 0.1, 1.0);

        float x = 0;

        while (!glfwWindowShouldClose(gameWindow->window))
        {
            x += 0.001;
            update();

            entityTwo->updateComponents();
        }

        glfwTerminate();

        delete entityOne;
        delete entityTwo;

        systems::ScriptingSystem::destroy();
    }

    void Game::update()
    {
        systems::ScriptingSystem::instance()->updateAll();
        systems::RenderSystem::instance()->render();

        if (keysPressed[GLFW_KEY_ESCAPE] == true)
        {
            giz::logger::logInfo("Escape pressed, gracefully exiting");
            glfwSetWindowShouldClose(gameWindow->window, 1);
        }

        glfwSwapBuffers(gameWindow->window);
        glfwPollEvents();

        time = glfwGetTime();
    }

    void Game::onCursorMove(double x, double y)
    {
        mousePosition.x = x;
        mousePosition.y = y;
    }

    void Game::onKeyPress(int key, int scancode, int action)
    {
        keysPressed[key] = true;
    }

    void Game::onKeyRelease(int key, int scancode, int action)
    {
        keysPressed[key] = false;
    }
}