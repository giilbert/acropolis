#pragma once

#include <v8.h>
#include <libplatform/libplatform.h>
#include <vector>
#include <unordered_map>
#include "scripting/api/Vector3Api.h"

namespace giz
{
    namespace component
    {
        class Behavior;
    }

    namespace scripting
    {
        class ModuleApiDeclaration;
    }

    namespace systems
    {
        class ScriptingSystem
        {
            giz::scripting::ModuleApiDeclaration *m_ModuleApiDeclaration;
            std::unique_ptr<v8::Platform> m_Platform;
            v8::Isolate *m_Isolate;
            v8::Isolate::CreateParams m_CreateParams;
            v8::Global<v8::Context> m_GlobalContext;

            unsigned int m_CurrentId = 0;
            std::unordered_map<unsigned int, giz::component::Behavior *> m_AttachedBehaviors;

            // maps a module's name to that module
            // used by the module resolver to resolve modules
            std::unordered_map<std::string, v8::Global<v8::Module> *> m_Modules;

            static v8::MaybeLocal<v8::Module> ModuleResolutionCallback(v8::Local<v8::Context> context,
                                                                       v8::Local<v8::String> specifier,
                                                                       v8::Local<v8::FixedArray> import_assertions,
                                                                       v8::Local<v8::Module> referrer);
            void CreateSyntheticModules(v8::Local<v8::Context> context);

        protected:
            // also init
            ScriptingSystem();
            static ScriptingSystem *singleton;

        public:
            // singletons should not be cloneable
            ScriptingSystem(ScriptingSystem &other) = delete;
            // singletons should not be assignable
            void operator=(const ScriptingSystem &) = delete;
            // getter method for singleton
            static ScriptingSystem *Instance();
            static void Destroy();

            void UpdateAll();

            // assigns and unassign each script an id to track which script goes with which entity
            void AttachScript(giz::component::Behavior *behavior);
            void DetachScript(giz::component::Behavior *behavior);
        };
    }
}