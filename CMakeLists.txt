cmake_minimum_required(VERSION 3.22.0)

project(giz)
cmake_policy(SET CMP0091 NEW)
set(CMAKE_CXX_STANDARD 17)

include_directories(${CMAKE_SOURCE_DIR}/modules)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})

add_definitions(-D_ITERATOR_DEBUG_LEVEL=0)
add_definitions(-DV8_COMPRESS_POINTERS)

add_executable(
  giz
  
  modules/main.cpp
)

add_subdirectory(modules/core)
add_subdirectory(modules/ecs)
add_subdirectory(modules/rendering)
add_subdirectory(modules/scripting)
add_subdirectory(modules/utils)

set_property(TARGET giz PROPERTY
             MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

file(GLOB GLSL_FILES
  "modules/shaders/*.vert"
  "modules/shaders/*.frag"
)
file(COPY ${GLSL_FILES} DESTINATION res/shader)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})

# glfw3
include_directories(${CMAKE_SOURCE_DIR}/thirdparty/glfw/include)
find_package(glfw3 3.3 REQUIRED)
target_link_libraries(giz ${CMAKE_SOURCE_DIR}/${GLFW_LIBRARY}/glfw3_mt.lib)

# glew
include_directories(${CMAKE_SOURCE_DIR}/thirdparty/glew/include)
find_package(glew 3.3 REQUIRED)
target_link_libraries(giz ${CMAKE_SOURCE_DIR}/${GLEW_LIBRARY}/glew32s.lib)

# v8
include_directories(${CMAKE_SOURCE_DIR}/thirdparty/v8/include)
find_package(v8 9.0 REQUIRED)
target_link_libraries(giz ${CMAKE_SOURCE_DIR}/${V8_LIBRARY}/v8_monolith.lib dbghelp.lib Winmm.lib)

#include glm
include_directories(${CMAKE_SOURCE_DIR}/thirdparty)

target_link_options(giz PUBLIC "/ignore:4099")

find_package(OpenGL REQUIRED)
target_link_libraries(giz OpenGL::GL)