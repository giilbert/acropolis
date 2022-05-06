#include "scripting/components/Behavior.h"

using giz::component::Behavior;

#include <iostream>

Behavior::Behavior(std::string s)
{
    m_Source = s;
}

Behavior::~Behavior()
{
    if (!m_Behavior.IsEmpty())
    {
        m_Behavior.Reset();
    }
}

void Behavior::Init()
{
    giz::systems::ScriptingSystem::Instance()->AttachScript(this);
}

void Behavior::Update()
{
}