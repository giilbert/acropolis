#include "logger.h"

namespace giz
{
    namespace logger
    {
        void Info(std::string string)
        {
            std::cout << "\x1B[44m INFO  \033[0m " << string << std::endl;
        }

        void Warn(std::string string)
        {
            std::cout << "\x1B[43;30m WARN  \033[0m " << string << std::endl;
        }

        void Error(std::string string)
        {
            std::cerr << "\x1B[41m ERROR \033[0m " << string << std::endl;
        }
    };
}