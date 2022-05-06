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
        static Game *Instance();

        Window *m_GameWindow;

        // window events
        void OnResize(int width, int height);
        void OnCursorMove(double x, double y);
        void OnKeyPress(int key, int scancode, int action);
        void onKeyRelease(int key, int scancode, int action);

        void Init();
        void Update();

    private:
        // time in seconds passed since init() was called
        float m_Time;

        // events
        glm::dvec2 m_MousePosition;
        std::map<int, bool> m_KeysPressed;
    };
}
