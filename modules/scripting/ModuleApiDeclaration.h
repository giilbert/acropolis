#pragma once

#include "v8.h"
#include "scripting/api/BehaviorApi.h"
#include "scripting/api/Vector3Api.h"

namespace giz
{
    namespace scripting
    {
        class ModuleApiDeclaration
        {
        public:
            ModuleApiDeclaration();
            ~ModuleApiDeclaration();
            v8::Local<v8::Module> CreateECSModule();

        private:
            giz::scripting::api::Behavior behaviorApi_;
            giz::scripting::api::Vector3 vector3Api_;
        };
    }
}