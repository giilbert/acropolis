#include <string>
#include "v8.h"

namespace giz
{
    namespace scripting
    {
        std::string fromV8String(v8::Local<v8::String> string);

        v8::Local<v8::String> toV8String(const char *string);
        v8::Local<v8::String> toV8String(std::string string);
    }
}