#include "assert.h"

void ASSERT_IF(bool condition, char *message)
{
    if (condition)
        giz::logger::Error(message);
}