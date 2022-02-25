#include "core/game.h"
#include "utils/logger.h"

int main(void)
{
    logger::logInfo("Hello from giz!");
    Game::instance()->init();

    return 0;
}