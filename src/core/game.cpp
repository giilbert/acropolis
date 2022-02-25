#include "game.h"

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

void Game::init()
{
    // std::cout << window << std::endl;

    gameWindow = Window();
    gameWindow.init();

    Renderer renderer = Renderer();

    std::vector<float> vertices(vertexData, vertexData + sizeof(vertexData) / sizeof(vertexData[0]));
    std::vector<unsigned int> indices(indexData, indexData + sizeof(indexData) / sizeof(indexData[0]));
    std::vector<float> normals(normalsData, normalsData + sizeof(normalsData) / sizeof(normalsData[0]));

    Mesh3D mesh(vertices, indices, normals);

    std::vector<char *> uniforms = {"projectionMatrix", "viewMatrix", "time"};

    // compile shaders
    Shader shader = Shader::loadFromFiles("res/shaders/basic.vert", "res/shaders/basic.frag", uniforms);
    shader.bind();

    // set uniforms
    glm::mat4 projectionMatrix = glm::perspective(glm::pi<float>() * 0.25f, 640.0f / 480.0f, 0.1f, 1000.0f);
    shader.setMatrix4x4(0, &projectionMatrix[0][0]);

    glBindVertexArray(mesh.vaoId);

    float time = 0;
    while (!glfwWindowShouldClose(gameWindow.window))
    {
        glm::mat4 viewMatrix = glm::lookAt(
            glm::vec3(sin(time) * 7, cos(time) * 7, cos(time) * 7),
            glm::vec3(0, 0, 0),
            glm::vec3(0, 1, 0));

        shader.setMatrix4x4(1, &viewMatrix[0][0]);
        shader.setFloat(2, time);

        renderer.render(gameWindow.window);

        time = glfwGetTime();
    }

    logger::logInfo("end");

    glfwTerminate();
}

void Game::update()
{
}