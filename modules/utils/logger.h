#pragma once
#include <iostream>
#include <string>

namespace giz
{
    namespace logger
    {
        void Info(std::string string);
        void Warn(std::string string);
        void Error(std::string string);
    };
}