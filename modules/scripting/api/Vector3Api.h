#include "v8.h"
#include "glm/glm.hpp"

namespace giz
{
    namespace scripting
    {
        namespace api
        {
            void initVector3Template();
            v8::Local<v8::Object> wrapVector3(glm::vec3 &vector);
        }
    }
}