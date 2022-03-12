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
        std::cout << width << height << std::endl;
    }

    void Game::init()
    {
        using component::Mesh;

        gameWindow = new Window();
        gameWindow->init();

        // init beforehand
        systems::RenderSystem::instance();

        std::vector<float> vertices(vertexData, vertexData + sizeof(vertexData) / sizeof(vertexData[0]));
        std::vector<unsigned int> indices(indexData, indexData + sizeof(indexData) / sizeof(indexData[0]));
        std::vector<float> normals(normalsData, normalsData + sizeof(normalsData) / sizeof(normalsData[0]));

        auto entityOne = std::make_unique<Entity>();
        auto meshOne = new Mesh(vertices, indices, normals);
        entityOne->addComponent(meshOne);

        glClearColor(0.1, 0.1, 0.1, 1.0);

        while (!glfwWindowShouldClose(gameWindow->window))
        {
            update();
        }

        glfwTerminate();
    }

    void Game::update()
    {
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