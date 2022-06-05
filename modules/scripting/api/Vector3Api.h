#pragma once

#include "v8.h"
#include "glm/glm.hpp"

namespace giz
{
    namespace scripting
    {
        namespace api
        {
            class Vector3
            {
            public:
                // creates template
                void Init();
                // destroys template
                void Destroy();

                static v8::Local<v8::Object> Wrap(glm::vec3 &vector);

                static v8::Global<v8::ObjectTemplate> m_Template;
            };
        }
    }
}