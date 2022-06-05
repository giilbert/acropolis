# tested on windows

import sys
import subprocess
import time
from pathlib import Path
from os import path, makedirs
import re
import json


# processes files in modules such that it can be included from #include
def gen_include():
    files = Path("modules").glob("*/meta.json")

    # loop over every meta.json in modules directory
    for f in files:
        clean_path = re.sub("(modules\\\\)|(\\\\meta.json)", "", str(f))
        # load contents into data
        file = open(f, "r")
        data = json.load(file)
        file.close()

        for include in data["transformIncludes"]:
            include_file = open(
                path.join(str(f).replace("\meta.json", ""), include), "r")
            include_data = include_file.read()
            include_file.close()

            new_file_directory = re.sub(
                r"[\\\\/][^(\\\\/)]+$", "",
                path.join("modules/generated", clean_path, include))

            if not path.exists(new_file_directory):
                makedirs(new_file_directory)

            new_file = open(
                path.join(new_file_directory,
                          re.search(r"[^(\\\/)]+$", include).group(0)), "w+")
            new_file.write('R"(')
            new_file.write(include_data)
            new_file.write(')"')


def build_debug():
    time_start = time.time()
    exit_code = subprocess.call(["cmake", "--build", "."])
    print(f"\nBuild finished in {round(time.time() - time_start, 2)} seconds")
    print("-----------------------------------------------")
    return exit_code


# debug
def run():
    exit_code = subprocess.call("./Debug/giz", cwd="./Debug")
    return exit_code


# argv[0] is the file name
# BUILD and run
if sys.argv[1] == "run":
    build_exit_code = build_debug()
    if build_exit_code == 1:
        exit(1)

    giz_exit_code = run()

    print("-----------------------------------------------")

    if giz_exit_code == 1:
        print("Exited abruptly")
    elif giz_exit_code == 3221225477:
        print("Access violation")
    else:
        # unhandled exit code
        print(f"Exit code: {hex(giz_exit_code)} ({giz_exit_code})")

if sys.argv[1] == "generate":
    gen_include()

if sys.argv[1] == "gen-api":
    name = input("Name: ")

    # create header file
    with open(f"modules/scripting/api/{name}Api.h", "w") as header:
        header.write(f"""
#pragma once

#include "v8.h"

namespace giz
{{
    namespace scripting
    {{
        namespace api
        {{
            class {name}
            {{
            public:
                // creates templates
                void Init();
                // destroys templates
                void Destroy();
                // static v8::Local<v8::Object> Wrap(giz::component::{name} &placeholder);
                static v8::Local<v8::Value> GetModuleApi();
                static v8::Global<v8::ObjectTemplate> m_ObjectTemplate;
                static v8::Global<v8::FunctionTemplate> m_FunctionTemplate;
            }};
        }}
    }}
}}
""")

    with open(f"modules/scripting/api/{name}Api.cpp", "w") as source:
        source.write(f"""
#include "scripting/api/{name}Api.h"
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

Global<ObjectTemplate> api::{name}::m_ObjectTemplate;
Global<FunctionTemplate> api::{name}::m_FunctionTemplate;

void api::{name}::Init()
{{
}}

void api::{name}::Destroy()
{{
    m_FunctionTemplate.Reset();
    m_ObjectTemplate.Reset();
}}

Local<Value> api::{name}::GetModuleApi()
{{
    Isolate *isolate = Isolate::GetCurrent();
    Local<Context> context = isolate->GetCurrentContext();
}}
""")
