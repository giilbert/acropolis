cmake_minimum_required(VERSION 3.22.0)

project(giz)

include_directories(${CMAKE_SOURCE_DIR}/src)

add_executable(
  giz
  
  src/main.cpp
  src/core/game.cpp
  src/core/window.cpp
  
  src/renderer/renderer.cpp
  src/renderer/shader.cpp
  src/renderer/vbo.cpp
  src/renderer/3d/mesh.cpp
  src/utils/assert.cpp
  src/utils/logger.cpp
)

file(GLOB GLSL_FILES
  "src/shaders/*.vert"
  "src/shaders/*.frag"
)
file(COPY ${GLSL_FILES} DESTINATION res/shader)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake ${CMAKE_MODULE_PATH})

include_directories(${CMAKE_SOURCE_DIR}/thirdparty/glfw/include)
find_package(glfw3 3.3 REQUIRED)
target_link_libraries(giz ${CMAKE_SOURCE_DIR}/${GLFW_LIBRARY}/glfw3.lib)

include_directories(${CMAKE_SOURCE_DIR}/thirdparty/glew/include)
find_package(glew 3.3 REQUIRED)
target_link_libraries(giz ${CMAKE_SOURCE_DIR}/${GLEW_LIBRARY}/glew32s.lib)

#include glm
include_directories(${CMAKE_SOURCE_DIR}/thirdparty)

find_package(OpenGL REQUIRED)
target_link_libraries(giz OpenGL::GL)