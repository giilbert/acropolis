#include "utils/logger.h"
#include <chrono>

namespace giz
{
    namespace profile
    {
        static std::chrono::steady_clock::time_point m_StartTime;
        void Start();
        void End(const char *description);
    }
}