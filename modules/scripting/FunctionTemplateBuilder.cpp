#include "scripting/FunctionTemplateBuilder.h"
#include "utils/string.h"

using giz::scripting::FunctionTemplateBuilder;
using giz::scripting::toV8String;
using v8::Context;
using v8::Data;
using v8::Function;
using v8::FunctionCallback;
using v8::FunctionTemplate;
using v8::Isolate;
using v8::Local;

FunctionTemplateBuilder::FunctionTemplateBuilder()
{
    template_ = FunctionTemplate::New(Isolate::GetCurrent());
}

FunctionTemplateBuilder FunctionTemplateBuilder::SetConstructor(FunctionCallback callback)
{
    template_->SetCallHandler(callback);
    return *this;
}

FunctionTemplateBuilder FunctionTemplateBuilder::SetPrototypeProperty(const char *name, Local<Data> value)
{
    template_->PrototypeTemplate()->Set(toV8String(name), value);
    return *this;
}

Local<FunctionTemplate> FunctionTemplateBuilder::Build()
{
    return template_;
}

Local<Function> FunctionTemplateBuilder::BuildFunction()
{
    return template_->GetFunction(Isolate::GetCurrent()->GetCurrentContext()).ToLocalChecked();
}