#pragma once
#include <iostream>

namespace giz
{
    namespace logger
    {
        void logInfo(char *string);
        void logWarning(char *string);
        void logError(char *string);
    };
}