#include "assert.h"

void ASSERT_IF(bool condition, char *message)
{
    if (condition)
        logger::logError(message);
}