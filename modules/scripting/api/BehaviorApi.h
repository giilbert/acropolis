#pragma once

#include "v8.h"
#include "scripting/components/Behavior.h"

namespace giz
{
    namespace scripting
    {
        namespace api
        {
            class Behavior
            {
            public:
                // creates templates
                void Init();
                // destroys templates
                void Destroy();

                // static v8::Local<v8::Object> Wrap(giz::component::Behavior &behavior);

                static v8::Local<v8::Value> GetModuleApi();
                static v8::Global<v8::ObjectTemplate> m_ObjectTemplate;
                static v8::Global<v8::FunctionTemplate> m_FunctionTemplate;
            };
        }
    }
}