#pragma once
#include <iostream>
#include <string>

namespace giz
{
    namespace logger
    {
        void logInfo(std::string string);
        void logWarning(std::string string);
        void logError(std::string string);
    };
}