#include "ecs/systems/ScriptingSystem.h"
#include "scripting/components/Behavior.h"
#include <iostream>
#include <chrono>
#include "utils/profile.h"
#include "utils/string.h"
#include "scripting/ObjectTemplateBuilder.h"
#include "scripting/FunctionTemplateBuilder.h"
#include "scripting/ModuleApiDeclaration.h"
#include "scripting/api/EntityApi.h"

// TODO: this
using namespace v8;
using namespace giz::scripting;
using giz::scripting::FunctionTemplateBuilder;
using giz::scripting::ModuleApiDeclaration;
using giz::scripting::ObjectTemplateBuilder;
using giz::systems::ScriptingSystem;

ScriptingSystem *ScriptingSystem::singleton = nullptr;

void logCallback(const FunctionCallbackInfo<Value> &args)
{
    if (args.Length() < 1)
        return;

    Isolate *isolate = args.GetIsolate();
    HandleScope scope(isolate);
    Local<Value> arg = args[0];
    String::Utf8Value value(isolate, arg);

    std::cout << *value << "\n";
}

Local<Context> createGlobalContext(Isolate *isolate)
{
    // create a template for the global object
    Local<ObjectTemplate> global = ObjectTemplate::New(isolate);
    // bind 'print' function to the c++ callback
    global->Set(
        String::NewFromUtf8(isolate, "print").ToLocalChecked(),
        FunctionTemplate::New(isolate, logCallback));

    return Context::New(isolate, NULL, global);
}

ScriptingSystem::ScriptingSystem()
{
    m_Platform = platform::NewDefaultPlatform();

    V8::InitializePlatform(m_Platform.get());
    V8::Initialize();
    // create an isolate and make it current
    m_CreateParams.array_buffer_allocator =
        v8::ArrayBuffer::Allocator::NewDefaultAllocator();
    m_Isolate = v8::Isolate::New(m_CreateParams);
    m_Isolate->Enter();

    v8::HandleScope handle_scope(m_Isolate);
    Local<Context> context = createGlobalContext(m_Isolate);

    m_ModuleApiDeclaration = new ModuleApiDeclaration();

    // create es modules used for imports
    CreateSyntheticModules(context);

    m_GlobalContext.Reset(m_Isolate, context);
}

void ScriptingSystem::Destroy()
{
    auto instance = ScriptingSystem::Instance();

    delete instance->m_ModuleApiDeclaration;

    auto isolate = instance->m_Isolate;
    auto createParams = instance->m_CreateParams;

    // dispose persistent handles
    // ScriptingSystem::instance()->globalContext.Reset();

    isolate->Exit();
    // dispose and clean up v8 javascript things
    isolate->Dispose();
    V8::Dispose();
    V8::ShutdownPlatform();
    delete createParams.array_buffer_allocator;
}

// TODO: refactor into seperate files
// TODO: clean up

void getTime(const FunctionCallbackInfo<Value> &info)
{
    info.GetReturnValue().Set(glfwGetTime());
}

void ScriptingSystem::CreateSyntheticModules(Local<Context> context)
{
    Local<Module> ecsModule = m_ModuleApiDeclaration->CreateECSModule();
    ecsModule->InstantiateModule(context, ScriptingSystem::ModuleResolutionCallback);

    Local<Module> gameModule = Module::CreateSyntheticModule(
        m_Isolate,
        String::NewFromUtf8(m_Isolate, "@giz/game").ToLocalChecked(),
        {String::NewFromUtf8(m_Isolate, "now").ToLocalChecked()},
        [](Local<Context> context, Local<Module> module) -> MaybeLocal<Value>
        {
            auto isolate = context->GetIsolate();
            auto nowCallback = Function::New(context, getTime).ToLocalChecked();

            module->SetSyntheticModuleExport(
                String::NewFromUtf8(isolate, "now").ToLocalChecked(),
                nowCallback);

            return MaybeLocal<Value>(True(isolate));
        });
    gameModule->InstantiateModule(context, ScriptingSystem::ModuleResolutionCallback);

    Global<Module> *globalEcsModule = new Global<Module>();
    globalEcsModule->Reset(m_Isolate, ecsModule);

    Global<Module> *globalGameModule = new Global<Module>();
    globalGameModule->Reset(m_Isolate, gameModule);

    m_Modules["@giz/ecs"] = globalEcsModule;
    m_Modules["@giz/game"] = globalGameModule;
}

MaybeLocal<Module> ScriptingSystem::ModuleResolutionCallback(Local<Context> context, Local<String> specifier,
                                                             Local<FixedArray> import_assertions, Local<Module> referrer)
{
    auto isolate = context->GetIsolate();
    auto modules = ScriptingSystem::Instance()->m_Modules;
    String::Utf8Value val(context->GetIsolate(), specifier);
    std::string request(*val, val.length());

    try
    {
        return modules.at(request)->Get(isolate);
    }
    catch (const std::out_of_range &e)
    {
        // TODO: better error handling
        logger::Error("Module " + request + " not found.  Crashing ..");
    }

    return MaybeLocal<Module>();
}

void ScriptingSystem::AttachScript(giz::component::Behavior *behavior)
{
    int scriptId = m_CurrentId;
    m_CurrentId += 1;
    // set the id of the behavior
    behavior->m_Id = scriptId;
    // keep track of it in the map
    m_AttachedBehaviors[scriptId] = behavior;

    // create contexts and scopes
    v8::HandleScope handle_scope(m_Isolate);
    v8::Local<Context> context = Local<Context>::New(m_Isolate, m_GlobalContext);
    Context::Scope contextScope(context);

    // create origin info for the script
    ScriptOrigin origin(String::NewFromUtf8(m_Isolate, "test").ToLocalChecked(), // specifier
                        Integer::New(m_Isolate, 0),                              // line offset
                        Integer::New(m_Isolate, 0),                              // column offset
                        False(m_Isolate),                                        // is cross origin
                        Integer::New(m_Isolate, scriptId),                       // script id
                        Local<Value>(),                                          // source map URL
                        True(m_Isolate),                                         // is opaque
                        False(m_Isolate),                                        // is WASM
                        True(m_Isolate));                                        // is ES6 module

    // compile the script
    // TODO: cache the compiled scripts so no need to recompile every time a script component is instantiated
    Local<String> scriptSourceRaw =
        String::NewFromUtf8(
            m_Isolate, behavior->m_Source.c_str(),
            NewStringType::kNormal)
            .ToLocalChecked();
    ScriptCompiler::Source source(scriptSourceRaw, origin);
    Local<Module> module = ScriptCompiler::CompileModule(m_Isolate, &source).ToLocalChecked();

    // resolve modules in the script
    module->InstantiateModule(context, ScriptingSystem::ModuleResolutionCallback);

    Local<Value> returnValue = module->Evaluate(context).ToLocalChecked();
    Local<Object> returnedBehavior = returnValue->ToObject(context).ToLocalChecked();

    Local<Value> arguments[1];
    arguments[0] = api::Entity::Wrap(*behavior->m_Entity);

    // creates an instance of the behavior
    auto instance = returnedBehavior->CallAsConstructor(context, 1, arguments)
                        .ToLocalChecked()
                        ->ToObject(context)
                        .ToLocalChecked();
    Local<Object> updateFunction =
        instance->Get(context, toV8String("update"))
            .ToLocalChecked()
            ->ToObject(context)
            .ToLocalChecked();

    behavior->m_Behavior.Reset(m_Isolate, instance);
    behavior->m_UpdateFunction.Reset(m_Isolate, updateFunction);
}

void ScriptingSystem::DetachScript(giz::component::Behavior *behavior)
{
    behavior->m_Behavior.Reset();
}

void ScriptingSystem::UpdateAll()
{
    giz::profile::Start();
    HandleScope handleScope(m_Isolate);
    Local<Context> context = Local<Context>::New(m_Isolate, m_GlobalContext);

    for (const auto [i, behavior] : m_AttachedBehaviors)
    {
        Local<Object> instance = behavior->m_Behavior.Get(m_Isolate);
        behavior->m_UpdateFunction.Get(m_Isolate)
            ->CallAsFunction(context, instance, 0, nullptr);
        behavior->m_Entity->m_Transform.UpdateTransform();
    }

    giz::profile::End("ScriptingSystem::UpdateAll");
}

ScriptingSystem *ScriptingSystem::Instance()
{
    if (singleton == nullptr)
    {
        singleton = new ScriptingSystem();
    }

    return singleton;
}