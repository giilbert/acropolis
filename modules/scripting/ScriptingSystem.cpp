#include "ecs/systems/ScriptingSystem.h"
#include "scripting/components/Behavior.h"
#include <iostream>
#include <chrono>
#include "utils/profile.h"

using namespace v8;
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
    platform = platform::NewDefaultPlatform();

    V8::InitializePlatform(platform.get());
    V8::Initialize();
    // create an isolate and make it current
    create_params.array_buffer_allocator =
        v8::ArrayBuffer::Allocator::NewDefaultAllocator();
    isolate = v8::Isolate::New(create_params);
    isolate->Enter();

    v8::HandleScope handle_scope(isolate);
    Local<Context> context = createGlobalContext(isolate);

    // create es modules used for imports
    createSyntheticModules(context);

    globalContext.Reset(isolate, context);
}

void ScriptingSystem::destroy()
{

    auto isolate = ScriptingSystem::instance()->isolate;
    auto createParams = ScriptingSystem::instance()->create_params;

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

// VECTOR 3

void getVectorX(Local<String> property,
                const PropertyCallbackInfo<Value> &info)
{
    Local<External> wrap = Local<External>::Cast(info.Holder()->GetInternalField(0));
    glm::vec3 vectorValue = *static_cast<glm::vec3 *>(wrap->Value());
    info.GetReturnValue().Set(vectorValue.x);
}

void setVectorX(Local<String> property, Local<Value> value,
                const PropertyCallbackInfo<void> &info)
{
    auto context = Isolate::GetCurrent()->GetCurrentContext();
    Local<External> wrap = Local<External>::Cast(info.Holder()->GetInternalField(0));
    float newValue = value->NumberValue(context).ToChecked();
    static_cast<glm::vec3 *>(wrap->Value())->x = newValue;
}

void getVectorY(Local<String> property,
                const PropertyCallbackInfo<Value> &info)
{
    Local<External> wrap = Local<External>::Cast(info.Holder()->GetInternalField(0));
    glm::vec3 vectorValue = *static_cast<glm::vec3 *>(wrap->Value());
    info.GetReturnValue().Set(vectorValue.y);
}

void setVectorY(Local<String> property, Local<Value> value,
                const PropertyCallbackInfo<void> &info)
{
    auto context = Isolate::GetCurrent()->GetCurrentContext();
    Local<External> wrap = Local<External>::Cast(info.Holder()->GetInternalField(0));
    float newValue = value->NumberValue(context).ToChecked();
    static_cast<glm::vec3 *>(wrap->Value())->y = newValue;
}

void getVectorZ(Local<String> property,
                const PropertyCallbackInfo<Value> &info)
{
    Local<External> wrap = Local<External>::Cast(info.Holder()->GetInternalField(0));
    glm::vec3 vectorValue = *static_cast<glm::vec3 *>(wrap->Value());
    info.GetReturnValue().Set(vectorValue.z);
}

void setVectorZ(Local<String> property, Local<Value> value,
                const PropertyCallbackInfo<void> &info)
{
    auto context = Isolate::GetCurrent()->GetCurrentContext();
    Local<External> wrap = Local<External>::Cast(info.Holder()->GetInternalField(0));
    float newValue = value->NumberValue(context).ToChecked();
    static_cast<glm::vec3 *>(wrap->Value())->z = newValue;
}

Local<Object> wrapVector3(glm::vec3 &vector)
{
    auto isolate = Isolate::GetCurrent();
    auto context = isolate->GetCurrentContext();
    EscapableHandleScope handleScope(isolate);

    Local<ObjectTemplate> vectorTemplate = ObjectTemplate::New(isolate);
    vectorTemplate->SetInternalFieldCount(1);

    vectorTemplate->SetAccessor(String::NewFromUtf8(isolate, "x").ToLocalChecked(),
                                getVectorX, setVectorX);
    vectorTemplate->SetAccessor(String::NewFromUtf8(isolate, "y").ToLocalChecked(),
                                getVectorY, setVectorY);
    vectorTemplate->SetAccessor(String::NewFromUtf8(isolate, "z").ToLocalChecked(),
                                getVectorZ, setVectorZ);

    Local<Object> instance = vectorTemplate->NewInstance(context).ToLocalChecked();
    instance->SetInternalField(0, External::New(isolate, &vector));

    return handleScope.Escape(instance);
}

// END VECTOR3

void behaviorConstructor(const FunctionCallbackInfo<Value> &info)
{
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto object = info.This();
    auto entity = info[0];

    if (entity->IsNullOrUndefined())
    {
        giz::logger::logError("pass an entity into super");
        return;
    }

    object->Set(context, String::NewFromUtf8(isolate, "entity").ToLocalChecked(), entity);

    // now set a bunch of aliases
    auto transform = entity
                         ->ToObject(context)
                         .ToLocalChecked()
                         ->Get(context, String::NewFromUtf8(isolate, "transform").ToLocalChecked())
                         .ToLocalChecked();
    auto position = transform
                        ->ToObject(context)
                        .ToLocalChecked()
                        ->Get(context, String::NewFromUtf8(isolate, "position").ToLocalChecked())
                        .ToLocalChecked();
    object->Set(context, String::NewFromUtf8(isolate, "transform").ToLocalChecked(), transform);
    object->Set(context, String::NewFromUtf8(isolate, "position").ToLocalChecked(), position);
}

void getEntityTransform(Local<String> property,
                        const PropertyCallbackInfo<Value> &info)
{
    auto isolate = Isolate::GetCurrent();
    auto context = isolate->GetCurrentContext();
    Local<Object>
        self = info.Holder();
    Local<External> wrap = Local<External>::Cast(self->GetInternalField(0));
    giz::Transform &transform = *static_cast<giz::Transform *>(wrap->Value());

    // TODO: create templates ahead of time

    // giz::profile::start();
    Local<ObjectTemplate> transformTemplate = ObjectTemplate::New(isolate);
    transformTemplate->SetInternalFieldCount(1);
    // giz::profile::end("accessor");

    Local<Object> instance = transformTemplate->NewInstance(context).ToLocalChecked();
    instance->SetInternalField(0, External::New(isolate, &transform));

    instance->Set(context, String::NewFromUtf8(isolate, "position").ToLocalChecked(), wrapVector3(transform.position));

    info.GetReturnValue().Set(instance);
}

void setEntityTransform(v8::Local<v8::String> property, v8::Local<v8::Value> value,
                        const v8::PropertyCallbackInfo<void> &info)
{
    v8::Local<v8::Object> self = info.Holder();
    v8::Local<v8::External> wrap =
        v8::Local<v8::External>::Cast(self->GetInternalField(0));
    void *ptr = wrap->Value();

    String::Utf8Value str(Isolate::GetCurrent(), property);

    // static_cast<giz::Entity *>(ptr)->id = value->Int32Value(Isolate::GetCurrent()->GetCurrentContext()).ToChecked();
}

// Local<Object> wrapTransform(const giz::Transform &transform)
// {
//     auto isolate = Isolate::GetCurrent();
//     EscapableHandleScope handleScope(isolate);
//     auto context = isolate->GetCurrentContext();
//     Local<ObjectTemplate> position = ObjectTemplate::New(isolate);

//     position->SetInternalFieldCount(1);
//     position->SetAccessor(String::NewFromUtf8(isolate, ""));

//     // TODO: add rotation, scale, etc.
//     return handleScope.Escape(position);
// }

Local<Object> wrapEntity(giz::Entity &entity)
{
    auto isolate = Isolate::GetCurrent();
    auto context = isolate->GetCurrentContext();
    EscapableHandleScope handleScope(isolate);

    Local<ObjectTemplate> entityTemplate = ObjectTemplate::New(isolate);
    entityTemplate->SetInternalFieldCount(1);
    entityTemplate->SetAccessor(String::NewFromUtf8(isolate, "transform").ToLocalChecked(),
                                getEntityTransform, setEntityTransform);
    // TODO: add more properties of entity
    // this is just a test

    auto instance = entityTemplate->NewInstance(context).ToLocalChecked();
    instance->SetInternalField(0, External::New(isolate, &entity));

    return handleScope.Escape(instance);
}

void getTime(const FunctionCallbackInfo<Value> &info)
{
    info.GetReturnValue().Set(glfwGetTime());
}

void ScriptingSystem::createSyntheticModules(Local<Context> context)
{
    Local<Module> ecsModule = Module::CreateSyntheticModule(
        isolate,
        String::NewFromUtf8(isolate, "@giz/ecs").ToLocalChecked(),
        {String::NewFromUtf8(isolate, "Behavior").ToLocalChecked()},
        [](Local<Context> context, Local<Module> module) -> MaybeLocal<Value>
        {
            auto isolate = context->GetIsolate();
            auto behavior = Function::New(context, behaviorConstructor).ToLocalChecked();

            module->SetSyntheticModuleExport(
                String::NewFromUtf8(isolate, "Behavior").ToLocalChecked(),
                behavior);

            return MaybeLocal<Value>(True(isolate));
        });
    ecsModule->InstantiateModule(context, ScriptingSystem::moduleResolutionCallback);

    Local<Module> gameModule = Module::CreateSyntheticModule(
        isolate,
        String::NewFromUtf8(isolate, "@giz/game").ToLocalChecked(),
        {String::NewFromUtf8(isolate, "now").ToLocalChecked()},
        [](Local<Context> context, Local<Module> module) -> MaybeLocal<Value>
        {
            auto isolate = context->GetIsolate();
            auto nowCallback = Function::New(context, getTime).ToLocalChecked();

            module->SetSyntheticModuleExport(
                String::NewFromUtf8(isolate, "now").ToLocalChecked(),
                nowCallback);

            return MaybeLocal<Value>(True(isolate));
        });
    gameModule->InstantiateModule(context, ScriptingSystem::moduleResolutionCallback);

    Global<Module> *globalEcsModule = new Global<Module>();
    globalEcsModule->Reset(isolate, ecsModule);

    Global<Module> *globalGameModule = new Global<Module>();
    globalGameModule->Reset(isolate, gameModule);

    modules["@giz/ecs"] = globalEcsModule;
    modules["@giz/game"] = globalGameModule;
}

MaybeLocal<Module> ScriptingSystem::moduleResolutionCallback(Local<Context> context, Local<String> specifier,
                                                             Local<FixedArray> import_assertions, Local<Module> referrer)
{
    auto isolate = context->GetIsolate();
    auto modules = ScriptingSystem::instance()->modules;
    String::Utf8Value val(context->GetIsolate(), specifier);
    std::string request(*val, val.length());

    try
    {
        return modules.at(request)->Get(isolate);
    }
    catch (const std::out_of_range &e)
    {
        // TODO: better error handling
        logger::logError("Module " + request + " not found.  Crashing ..");
    }

    return MaybeLocal<Module>();
}

void ScriptingSystem::attachScript(giz::component::Behavior *behavior)
{
    int scriptId = currentId;
    currentId += 1;
    // set the id of the behavior
    behavior->id = scriptId;
    // keep track of it in the map
    attachedBehaviors[scriptId] = behavior;

    // create contexts and scopes
    v8::HandleScope handle_scope(isolate);
    v8::Local<Context> context = Local<Context>::New(isolate, globalContext);
    Context::Scope contextScope(context);

    // create origin info for the script
    ScriptOrigin origin(String::NewFromUtf8(isolate, "test").ToLocalChecked(), // specifier
                        Integer::New(isolate, 0),                              // line offset
                        Integer::New(isolate, 0),                              // column offset
                        False(isolate),                                        // is cross origin
                        Integer::New(isolate, scriptId),                       // script id
                        Local<Value>(),                                        // source map URL
                        True(isolate),                                         // is opaque
                        False(isolate),                                        // is WASM
                        True(isolate));                                        // is ES6 module

    // compile the script
    // TODO: cache the compiled scripts so no need to recompile every time a script component is instantiated
    Local<String> scriptSourceRaw =
        String::NewFromUtf8(
            isolate, behavior->source.c_str(),
            NewStringType::kNormal)
            .ToLocalChecked();
    ScriptCompiler::Source source(scriptSourceRaw, origin);
    Local<Module> module = ScriptCompiler::CompileModule(isolate, &source).ToLocalChecked();

    // resolve modules in the script
    module->InstantiateModule(context, ScriptingSystem::moduleResolutionCallback);

    Local<Value> returnValue = module->Evaluate(context).ToLocalChecked();
    Local<Object> returnedBehavior = returnValue->ToObject(context).ToLocalChecked();

    Local<Value> arguments[1];
    arguments[0] = wrapEntity(*behavior->entity);

    // creates an instance of the behavior
    auto instance = returnedBehavior->CallAsConstructor(context, 1, arguments)
                        .ToLocalChecked()
                        ->ToObject(context)
                        .ToLocalChecked();

    // // calls the update function
    // instance->Get(context, String::NewFromUtf8(isolate, "update").ToLocalChecked())
    //     .ToLocalChecked()
    //     ->ToObject(context)
    //     .ToLocalChecked()
    //     ->CallAsFunction(context, instance, 0, nullptr);

    // giz::profile::end("Create entity object");

    behavior->behavior.Reset(isolate, instance);
}

void ScriptingSystem::detachScript(giz::component::Behavior *behavior)
{
    behavior->behavior.Reset();
}

void ScriptingSystem::updateAll()
{
    HandleScope handleScope(isolate);
    Local<Context> context = Local<Context>::New(isolate, globalContext);

    for (const auto [i, behavior] : attachedBehaviors)
    {
        Local<Object> instance = behavior->behavior.Get(isolate);
        instance->Get(context, String::NewFromUtf8(isolate, "update").ToLocalChecked())
            .ToLocalChecked()
            ->ToObject(context)
            .ToLocalChecked()
            ->CallAsFunction(context, instance, 0, nullptr);
    }
}

ScriptingSystem *ScriptingSystem::instance()
{
    if (singleton == nullptr)
    {
        singleton = new ScriptingSystem();
    }

    return singleton;
}