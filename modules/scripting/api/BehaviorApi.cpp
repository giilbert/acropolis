#include "scripting/api/BehaviorApi.h"
#include "scripting/FunctionTemplateBuilder.h"
#include "utils/string.h"

using namespace giz::scripting;
using v8::Context;
using v8::FunctionCallbackInfo;
using v8::FunctionTemplate;
using v8::Global;
using v8::Isolate;
using v8::Local;
using v8::ObjectTemplate;
using v8::Value;

Global<ObjectTemplate> api::Behavior::m_ObjectTemplate;
Global<FunctionTemplate> api::Behavior::m_FunctionTemplate;

void behaviorConstructor(const FunctionCallbackInfo<Value> &info)
{
    auto isolate = info.GetIsolate();
    auto context = isolate->GetCurrentContext();
    auto object = info.This();
    auto entity = info[0];

    if (entity->IsNullOrUndefined())
    {
        giz::logger::Error("pass an entity into super");
        return;
    }

    object->Set(context, toV8String("entity"), entity);

    // now set a bunch of aliases
    auto transform = entity
                         ->ToObject(context)
                         .ToLocalChecked()
                         ->Get(context, toV8String("transform"))
                         .ToLocalChecked();
    auto position = transform
                        ->ToObject(context)
                        .ToLocalChecked()
                        ->Get(context, toV8String("position"))
                        .ToLocalChecked();
    object->Set(context, toV8String("transform"), transform);
    object->Set(context, toV8String("position"), position);
}

void api::Behavior::Init()
{
    FunctionTemplateBuilder builder;
    m_FunctionTemplate = Global<FunctionTemplate>(
        Isolate::GetCurrent(),
        builder.SetConstructor(behaviorConstructor).Build());
}

void api::Behavior::Destroy()
{
    m_FunctionTemplate.Reset();
    m_ObjectTemplate.Reset();
}

Local<Value> api::Behavior::GetModuleApi()
{
    Isolate *isolate = Isolate::GetCurrent();
    Local<Context> context = isolate->GetCurrentContext();

    return m_FunctionTemplate.Get(isolate)->GetFunction(context).ToLocalChecked();
}