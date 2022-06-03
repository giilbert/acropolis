#include "Vector3Api.h"
#include "scripting/ObjectTemplateBuilder.h"

using v8::Context;
using v8::EscapableHandleScope;
using v8::External;
using v8::Global;
using v8::Isolate;
using v8::Local;
using v8::Object;
using v8::ObjectTemplate;
using v8::PropertyCallbackInfo;
using v8::String;
using v8::Value;

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

namespace giz
{
    namespace scripting
    {
        namespace api
        {
            static Global<ObjectTemplate> vector3Template;

            void initVector3Template()
            {
                ObjectTemplateBuilder builder;
                Local<ObjectTemplate> vectorTemplate =
                    builder
                        .SetPropertyImpl("x", getVectorX, setVectorX)
                        .SetPropertyImpl("y", getVectorY, setVectorY)
                        .SetPropertyImpl("z", getVectorZ, setVectorZ)
                        .Build();
                vectorTemplate->SetInternalFieldCount(1);

                vector3Template = Global<ObjectTemplate>(Isolate::GetCurrent(), vectorTemplate);
            }

            Local<Object> wrapVector3(glm::vec3 &vector)
            {
                Isolate *isolate = Isolate::GetCurrent();
                Local<Context> context = isolate->GetCurrentContext();
                EscapableHandleScope handleScope(isolate);

                Local<Object> instance = vector3Template.Get(isolate)
                                             ->NewInstance(context)
                                             .ToLocalChecked();
                instance->SetInternalField(0, External::New(isolate, &vector));

                return handleScope.Escape(instance);
            }
        }
    }
}