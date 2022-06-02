#include "scripting/ObjectTemplateBuilder.h"
#include "utils/string.h"

using giz::scripting::ObjectTemplateBuilder;
using giz::scripting::toV8String;
using v8::AccessorGetterCallback;
using v8::AccessorSetterCallback;
using v8::Isolate;
using v8::Local;
using v8::ObjectTemplate;

ObjectTemplateBuilder::ObjectTemplateBuilder()
{
    Isolate *isolate = Isolate::GetCurrent();
    template_ = ObjectTemplate::New(isolate);
}

ObjectTemplateBuilder ObjectTemplateBuilder::SetPropertyImpl(
    const char *name,
    AccessorGetterCallback getter, AccessorSetterCallback setter)
{
    template_->SetAccessor(toV8String(name), getter, setter);
    return *this;
}

Local<ObjectTemplate> ObjectTemplateBuilder::Build()
{
    return template_;
}