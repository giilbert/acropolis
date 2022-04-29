#pragma once

#include <iostream>
#include <chrono>
#include <vector>
#include <map>

#include "rendering/components/Mesh.h"
#include "rendering/components/Camera.h"
#include "scripting/components/Behavior.h"
#include "ecs/systems/RenderSystem.h"
#include "ecs/systems/ScriptingSystem.h"
#include "utils/logger.h"
#include "core/Window.h"

namespace giz
{
    class Window;
    class Game
    {
    protected:
        Game();
        static Game *singleton;

    public:
        // singletons should not be cloneable
        Game(Game &other) = delete;
        // singletons should not be assignable
        void operator=(const Game &) = delete;
        // getter method for the Game singleton
        static Game *instance();

        Window *gameWindow;

        // window events
        void onResize(int width, int height);
        void onCursorMove(double x, double y);
        void onKeyPress(int key, int scancode, int action);
        void onKeyRelease(int key, int scancode, int action);

        void init();
        void update();

    private:
        // time in seconds passed since init() was called
        float time;

        // events
        glm::dvec2 mousePosition;
        std::map<int, bool> keysPressed;
    };
}
