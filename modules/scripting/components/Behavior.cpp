#include "scripting/components/Behavior.h"

using giz::component::Behavior;

#include <iostream>

Behavior::Behavior(std::string s)
{
    source = s;
}

Behavior::~Behavior()
{
    if (!behavior.IsEmpty())
    {
        behavior.Reset();
    }
}

void Behavior::init()
{
    giz::systems::ScriptingSystem::instance()->attachScript(this);
}

void Behavior::update()
{
}