#pragma once

#include "v8.h"
#include "ecs/Entity.h"

namespace giz
{
    namespace scripting
    {
        namespace api
        {
            class Entity
            {
            public:
                // creates templates
                void Init();
                // destroys templates
                void Destroy();

                static v8::Local<v8::Object> Wrap(giz::Entity &entity);
                // static v8::Local<v8::Value> GetModuleApi();

                static v8::Global<v8::ObjectTemplate> m_ObjectTemplate;
                static v8::Global<v8::FunctionTemplate> m_FunctionTemplate;
            };
        }
    }
}
