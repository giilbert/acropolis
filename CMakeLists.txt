cmake_minimum_required(VERSION 3.22.0)

project(giz)
cmake_policy(SET CMP0091 NEW)
set(CMAKE_CXX_STANDARD 17)

include_directories(${CMAKE_SOURCE_DIR}/modules)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} /MTd")

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

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)

add_subdirectory(${CMAKE_SOURCE_DIR}/dep/glfw)
add_subdirectory(${CMAKE_SOURCE_DIR}/dep/glad)
add_subdirectory(${CMAKE_SOURCE_DIR}/dep/glm)

TARGET_LINK_LIBRARIES(giz glfw)
TARGET_LINK_LIBRARIES(giz glad)
TARGET_LINK_LIBRARIES(giz glm)

# v8
include_directories(${CMAKE_SOURCE_DIR}/dep/v8/include)
find_package(v8 9.0 REQUIRED)
target_link_libraries(giz ${CMAKE_SOURCE_DIR}/${V8_LIBRARY}/v8_monolith.lib dbghelp.lib Winmm.lib)

# TODO: investiagate why these are happening
target_link_options(giz PUBLIC "/ignore:4099")
target_link_options(giz PUBLIC "/ignore:4217")
target_link_options(giz PUBLIC "/ignore:4098")

find_package(OpenGL REQUIRED)
target_link_libraries(giz OpenGL::GL)