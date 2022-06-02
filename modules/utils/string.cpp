#include "utils/string.h"

using v8::EscapableHandleScope;
using v8::Isolate;
using v8::Local;
using v8::String;

namespace giz
{
    namespace scripting
    {
        std::string fromV8String(Local<String> string)
        {
            Isolate *isolate = Isolate::GetCurrent();
            String::Utf8Value value(isolate, string);
            return *value;
        }

        Local<String> toV8String(const char *string)
        {
            Isolate *isolate = Isolate::GetCurrent();
            EscapableHandleScope escapableHandleScope(isolate);
            Local<String> v8String = v8::String::NewFromUtf8(isolate, string).ToLocalChecked();
            return escapableHandleScope.Escape(v8String);
        }

        Local<String> toV8String(std::string string)
        {
            Isolate *isolate = Isolate::GetCurrent();
            EscapableHandleScope escapableHandleScope(isolate);
            Local<String> v8String = v8::String::NewFromUtf8(isolate, string.c_str()).ToLocalChecked();
            return escapableHandleScope.Escape(v8String);
        }
    }
}