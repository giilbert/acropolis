
#include "scripting/api/EntityApi.h"
#include "scripting/ObjectTemplateBuilder.h"
#include "utils/string.h"
#include "ecs/Transform.h"
#include "scripting/api/Vector3Api.h"

using namespace giz::scripting;
using v8::Context;
using v8::EscapableHandleScope;
using v8::External;
using v8::FunctionCallbackInfo;
using v8::FunctionTemplate;
using v8::Global;
using v8::Isolate;
using v8::Local;
using v8::Object;
using v8::ObjectTemplate;
using v8::PropertyCallbackInfo;
using v8::String;
using v8::Value;

Global<ObjectTemplate> api::Entity::m_ObjectTemplate;
Global<FunctionTemplate> api::Entity::m_FunctionTemplate;

void getEntityTransform(Local<String> property,
                        const PropertyCallbackInfo<Value> &info)
{
    auto isolate = Isolate::GetCurrent();
    auto context = isolate->GetCurrentContext();
    Local<Object>
        self = info.Holder();
    Local<External> wrap = Local<External>::Cast(self->GetInternalField(0));
    giz::Transform &transform = *static_cast<giz::Transform *>(wrap->Value());

    Local<ObjectTemplate> transformTemplate = ObjectTemplate::New(isolate);
    transformTemplate->SetInternalFieldCount(1);

    Local<Object> instance = transformTemplate->NewInstance(context).ToLocalChecked();
    instance->SetInternalField(0, External::New(isolate, &transform));

    instance->Set(
        context,
        toV8String("position"),
        giz::scripting::api::Vector3::Wrap(transform.m_Position));

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
}

void api::Entity::Init()
{
    ObjectTemplateBuilder builder;
    Local<ObjectTemplate> entityTemplate =
        builder
            .SetPropertyImpl("transform", getEntityTransform, setEntityTransform)
            .Build();

    Isolate *isolate = Isolate::GetCurrent();
    m_ObjectTemplate = Global<ObjectTemplate>(isolate, entityTemplate);
}

void api::Entity::Destroy()
{
    m_FunctionTemplate.Reset();
    m_ObjectTemplate.Reset();
}

Local<Object> api::Entity::Wrap(giz::Entity &entity)
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

// Local<Value> api::Entity::GetModuleApi()
// {
//     Isolate *isolate = Isolate::GetCurrent();
//     Local<Context> context = isolate->GetCurrentContext();
// }
