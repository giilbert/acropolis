#include "ecs/components/BaseComponent.h"

#include "utils/logger.h"

namespace giz
{
    namespace component
    {
        Base::~Base()
        {
            logger::logInfo("Base component destructor called");
        }
    }
}