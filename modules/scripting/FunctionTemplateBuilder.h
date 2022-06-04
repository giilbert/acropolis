#include <v8.h>

namespace giz
{
    namespace scripting
    {
        class FunctionTemplateBuilder
        {
        public:
            FunctionTemplateBuilder();

            FunctionTemplateBuilder SetConstructor(v8::FunctionCallback callback);
            FunctionTemplateBuilder SetPrototypeProperty(const char *name, v8::Local<v8::Data> value);

            v8::Local<v8::FunctionTemplate> Build();
            v8::Local<v8::Function> BuildFunction();

        private:
            v8::FunctionCallback callback_;
            v8::Local<v8::FunctionTemplate> template_;
        };

    }
}