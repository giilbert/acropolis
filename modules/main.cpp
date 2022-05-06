#include "core/Game.h"
#include "utils/logger.h"

int main()
{
    giz::logger::Info("Hello from giz!");
    giz::Game::Instance()->Init();

    return 0;
}