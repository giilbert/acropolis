#include <v8.h>

namespace giz
{
    namespace scripting
    {
        class ObjectTemplateBuilder
        {
        public:
            ObjectTemplateBuilder();

            ObjectTemplateBuilder SetPropertyImpl(
                const char *name,
                v8::AccessorGetterCallback getter,
                v8::AccessorSetterCallback setter);

            // TODO: static void SetMethodImpl(const char *name);
            v8::Local<v8::ObjectTemplate> Build();

        private:
            v8::Local<v8::ObjectTemplate> template_;
        };

    }
}