#include "utils/logger.h"
#include <chrono>

namespace giz
{
    namespace profile
    {
        static std::chrono::steady_clock::time_point startTime;
        void start();
        void end(const char *description);
    }
}