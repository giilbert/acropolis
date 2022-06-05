#include "scripting/ModuleApiDeclaration.h"
#include "utils/string.h"
#include "scripting/FunctionTemplateBuilder.h"

#include "scripting/api/BehaviorApi.h"

using giz::scripting::ModuleApiDeclaration;
using v8::Context;
using v8::EscapableHandleScope;
using v8::Isolate;
using v8::Local;
using v8::MaybeLocal;
using v8::Module;
using v8::Value;

ModuleApiDeclaration::ModuleApiDeclaration()
{
    behaviorApi_.Init();
    vector3Api_.Init();
    entityApi_.Init();
}

ModuleApiDeclaration::~ModuleApiDeclaration()
{
    behaviorApi_.Destroy();
    vector3Api_.Destroy();
    entityApi_.Destroy();
}

Local<Module> ModuleApiDeclaration::CreateECSModule()
{
    Isolate *isolate = Isolate::GetCurrent();
    return Module::CreateSyntheticModule(
        isolate,
        toV8String("@giz/ecs"),
        {toV8String("Behavior")},
        [](Local<Context> context, Local<Module> module) -> MaybeLocal<Value>
        {
            auto isolate = context->GetIsolate();

            module->SetSyntheticModuleExport(toV8String("Behavior"), api::Behavior::GetModuleApi());

            return MaybeLocal<Value>(True(isolate));
        });
}