#pragma once

#include <v8.h>
#include <libplatform/libplatform.h>
#include <vector>
#include <unordered_map>

namespace giz
{
    namespace component
    {
        class Behavior;
    }

    namespace systems
    {
        class ScriptingSystem
        {
            std::unique_ptr<v8::Platform> platform;
            v8::Isolate *isolate;
            v8::Isolate::CreateParams create_params;
            v8::Global<v8::Context> globalContext;

            unsigned int currentId = 0;
            std::unordered_map<unsigned int, giz::component::Behavior *> attachedBehaviors;

            // maps a module's name to that module
            // used by the module resolver to resolve modules
            std::unordered_map<std::string, v8::Global<v8::Module> *> modules;

            static v8::MaybeLocal<v8::Module> moduleResolutionCallback(v8::Local<v8::Context> context,
                                                                       v8::Local<v8::String> specifier,
                                                                       v8::Local<v8::FixedArray> import_assertions,
                                                                       v8::Local<v8::Module> referrer);
            void createSyntheticModules(v8::Local<v8::Context> context);

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
            static ScriptingSystem *instance();
            static void destroy();

            void updateAll();

            // assigns and unassign each script an id to track which script goes with which entity
            void attachScript(giz::component::Behavior *behavior);
            void detachScript(giz::component::Behavior *behavior);
        };
    }
}