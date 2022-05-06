#include "utils/profile.h"

namespace giz
{
    namespace profile
    {
        void Start()
        {
            giz::profile::m_StartTime = std::chrono::high_resolution_clock::now();
        }

        void End(const char *description)
        {
            using milli = std::chrono::milliseconds;
            using nano = std::chrono::nanoseconds;

            auto endTime = std::chrono::high_resolution_clock::now();
            auto difference = endTime - giz::profile::m_StartTime;

            std::cout << description
                      << " took "
                      << std::chrono::duration_cast<nano>(difference).count()
                      << " nanoseconds\n";
        }
    }
}