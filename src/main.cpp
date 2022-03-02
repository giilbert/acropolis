#include "core/Game.h"
#include "utils/logger.h"

int main(void)
{
    giz::logger::logInfo("Hello from giz!");
    giz::Game::instance()->init();

    return 0;
}