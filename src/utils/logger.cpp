#include "logger.h"

namespace logger
{
    void logInfo(char *string)
    {
        std::cout << "\x1B[44m INFO  \033[0m " << string << std::endl;
    }

    void logWarning(char *string)
    {
        std::cout << "\x1B[43;30m WARN  \033[0m " << string << std::endl;
    }

    void logError(char *string)
    {
        std::cout << "\x1B[41m ERROR \033[0m " << string << std::endl;
    }
}