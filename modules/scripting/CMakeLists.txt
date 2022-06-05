target_sources(
  giz
  PRIVATE
  components/Behavior.cpp
  ScriptingSystem.cpp
  ObjectTemplateBuilder.cpp
  FunctionTemplateBuilder.cpp
  ModuleApiDeclaration.cpp
  
  api/Vector3Api.cpp
  api/BehaviorApi.cpp
)